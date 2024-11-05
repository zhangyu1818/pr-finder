import { graphql } from '@octokit/graphql';
import { Command } from 'commander';

const program = new Command();

program
  .option('-t, --token <token>', 'GitHub personal access token')
  .option('-r, --repo <repo>', 'Repository to query (e.g. owner/repo)')
  .option('-a, --author <author>', 'Filter by author')
  .option(
    '-s, --status <status>',
    'Status of PRs: open, closed, merged',
    'merged'
  )
  .option(
    '--start-date <startDate>',
    'Start date for the query (e.g. 2024-01-01)'
  )
  .option('--end-date <endDate>', 'End date for the query (e.g. 2024-01-31)')
  .option('--group-by <groupBy>', 'Grouping mode: day, week, month', 'day')
  .parse(process.argv);

const options = program.opts();

if (!options.token || !options.repo) {
  console.error('Token and repository are required.');
  process.exit(1);
}

const gql = graphql.defaults({
  headers: {
    authorization: `token ${options.token}`,
  },
});

interface Response {
  search: {
    issueCount: number;
    edges: {
      cursor: string;
      node: {
        number: number;
        title: string;
        createdAt: string;
      };
    }[];
    pageInfo: {
      endCursor: string;
      hasNextPage: boolean;
    };
  };
}

interface Result {
  number: number;
  title: string;
  date: string;
}

function getQueryDateRange(startDate?: string, endDate?: string): string {
  if (startDate && endDate) {
    return `created:${startDate}..${endDate}`;
  } else if (startDate) {
    return `created:>=${startDate}`;
  } else if (endDate) {
    return `created:<=${endDate}`;
  } else {
    throw new Error('Start date or end date must be specified.');
  }
}

function getFormattedDate(date: Date, groupBy: string): string {
  const options: Intl.DateTimeFormatOptions = {
    year: 'numeric',
    timeZone: Intl.DateTimeFormat().resolvedOptions().timeZone,
  };

  switch (groupBy) {
    case 'week':
      const startOfWeek = new Date(date);
      startOfWeek.setDate(date.getDate() - date.getDay());
      options.month = '2-digit';
      options.day = '2-digit';
      return `Week of ${new Intl.DateTimeFormat(undefined, options).format(
        startOfWeek
      )}`;
    case 'month':
      options.month = '2-digit';
      return new Intl.DateTimeFormat(undefined, options).format(date);
    case 'day':
    default:
      options.month = '2-digit';
      options.day = '2-digit';
      return new Intl.DateTimeFormat(undefined, options).format(date);
  }
}

async function fetchPullRequests(cursor?: string) {
  const dateRangeFilter = getQueryDateRange(options.startDate, options.endDate);
  const authorFilter = options.author ? `author:${options.author}` : '';
  const stateFilter =
    options.status === 'open' || options.status === 'closed'
      ? `is:${options.status}`
      : 'is:pr is:merged';

  const query = `{
    search(query: "repo:${
      options.repo
    } ${stateFilter} ${authorFilter} ${dateRangeFilter}", type: ISSUE, first: 100${
    cursor ? `, after: \"${cursor}\"` : ''
  }) {
      issueCount
      edges {
        cursor
        node {
          ... on PullRequest {
            number
            title
            createdAt
          }
        }
      }
      pageInfo {
        endCursor
        hasNextPage
      }
    }
  }`;

  return gql<Response>(query);
}

async function main() {
  let cursor: string | undefined = undefined;
  let hasNextPage = true;
  const allEdges: Response['search']['edges'] = [];

  try {
    while (hasNextPage) {
      const { search } = await fetchPullRequests(cursor);
      allEdges.push(...search.edges);
      cursor = search.pageInfo.endCursor;
      hasNextPage = search.pageInfo.hasNextPage;
    }

    if (allEdges.length === 0) {
      console.log('No results found.');
      return;
    }

    const prs = Object.groupBy.call(
      null,
      allEdges
        .toSorted(
          (a, b) => +new Date(b.node.createdAt) - +new Date(a.node.createdAt)
        )
        .map((v) => {
          const date = new Date(v.node.createdAt);
          const number = v.node.number;
          const title = v.node.title;
          const formattedDate = getFormattedDate(date, options.groupBy);

          return {
            number,
            title,
            date: formattedDate,
          };
        }),
      (v) => (v as Result).date
    ) as Record<string, Result[]>;

    Object.entries(prs).forEach(([key, values]) => {
      console.log(key);
      values.forEach((v) => {
        console.log(v.title, `#${v.number}`);
      });
      console.log('\n');
    });
  } catch (error) {
    console.error('Error fetching data:', error);
  }
}

main();
