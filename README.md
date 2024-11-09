# PR Finder

如果公司代码仓库是GitHub，那汇报工作的时候可以通过此工具查询上班干了啥。

This tool allows you to query Pull Requests (PRs) from a GitHub repository based on various filters, such as author, status, date range, and more. It uses the GitHub GraphQL API to retrieve and display the requested information in a formatted output.

> This tool was initially written in TypeScript([branch](https://github.com/zhangyu1818/pr-finder/tree/typescript)) and compiled into a binary using Bun. However, the compiled output was extremely large (over 100MB). After rewriting it in Rust, the binary size was reduced to just 2MB.

## How to Use

### Prerequisites

- You need a GitHub personal access token with permissions to read repository data.

### Installation

1. Download the binary package for your platform from the [Releases](https://github.com/zhangyu1818/pr-finder/releases) section.
2. Ensure the binary is executable:

```bash
chmod +x pr-finder
```

### Running the Tool

Run the tool from your terminal, providing the required options:

```bash
./pr-finder -t <your_github_token> -r <owner/repo> [options]
```

### Options

```bash
./pr-finder --help
```

- `-t, --token <token>`: **Required**. Your GitHub personal access token.
- `-r, --repo <repo>`: **Required**. The repository to query in the format `owner/repo` (e.g., `octocat/Hello-World`).
- `-a, --author <author>`: Filter PRs by a specific author.
- `-s, --status <status>`: Status of PRs to query. Options: `open`, `closed`, `merged`.
- `-g, --group-by <groupBy>`: Group results by `day`, `week`, or `month`. Default is `day`.
- `--start-date <startDate>`: Start date for filtering PRs (e.g., `2024-01-01`).
- `--end-date <endDate>`: End date for filtering PRs (e.g., `2024-01-31`).

### Example Usage

Query all merged PRs in the repository `octocat/Hello-World` created by the author `octocat` between `2024-10-01` and `2024-10-15`:

```bash
./pr-finder -t your_github_token -r octocat/Hello-World -a octocat --start-date 2024-10-01 --end-date 2024-10-15 -s merged
```

### Output

The tool groups the PRs by the specified time period (`day`, `week`, or `month`) and displays each PR title.

## Notes

- The tool uses pagination to handle more than 100 results by iteratively querying additional pages.
- Ensure your GitHub token has sufficient permissions to read the repository data.
