mod cli;
mod filter;
mod graphql;
mod models;
mod utils;

use clap::Parser;
use std::{error::Error, process};
use colored::Colorize;

use cli::Args;
use graphql::query;
use models::SearchResult;

fn main() -> Result<(), Box<dyn Error>> {
    let Args {
        token,
        repo,
        author,
        status,
        group_by,
        start_date,
        end_date,
    } = Args::parse();

    let client = graphql::create_client(token)?;

    let author_filter = filter::get_author_filter(author);
    let date_filter = filter::get_date_filter(start_date, end_date);
    let status_filter = filter::get_status_filter(status);
    let query = format!(
        "repo:{} type:pr {} {} {}",
        repo, status_filter, date_filter, author_filter
    );

    let mut cursor: Option<String> = None;
    let mut has_next_page = true;
    let mut all_nodes: Vec<SearchResult> = Vec::new();

    while has_next_page {
        let query_body = graphql::create_query_body(query.clone(), cursor.clone());

        if let Some(data) = graphql::fetch_pull_requests(&client, query_body)?.data {
            let query::QuerySearch {
                nodes, page_info, ..
            } = data.search;

            if let Some(nodes) = nodes {
                all_nodes.extend(nodes.into_iter().filter_map(|node| {
                    if let Some(query::QuerySearchNodes::PullRequest(node)) = node {
                        let parsed_date = chrono::DateTime::parse_from_rfc3339(&node.created_at)
                            .map(|dt| dt.with_timezone(&chrono::Local))
                            .ok()?;

                        Some(SearchResult {
                            number: node.number,
                            title: node.title,
                            date: parsed_date,
                        })
                    } else {
                        None
                    }
                }));
            }

            has_next_page = page_info.has_next_page;
            cursor = page_info.end_cursor;
        } else {
            has_next_page = false;
        }
    }

    if all_nodes.is_empty() {
        println!("No results found.");
        process::exit(0);
    }

    let results = utils::create_map_with_group_by(all_nodes, group_by);

    let mut results: Vec<_> = results.into_iter().collect();

    results.sort_by(|(a, _), (b, _)| b.cmp(a));

    results.iter().for_each(|(grouped_date, nodes)| {
        println!("{}", grouped_date.red().bold().italic());
        nodes
            .into_iter()
            .for_each(|node| println!("{} #{}", node.title.trim_start(), node.number));
        println!();
    });

    Ok(())
}
