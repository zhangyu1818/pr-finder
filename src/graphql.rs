use graphql_client::{GraphQLQuery, QueryBody, Response};
use reqwest::blocking::Client;

const GITHUB_API_URL: &str = "https://api.github.com/graphql";
const USER_AGENT: &str = "graphql_client/0.14.0";

type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "queries/schema.docs.graphql",
    query_path = "queries/search.graphql",
    response_derives = "Debug"
)]
pub struct Query;

pub fn create_client(token: String) -> Result<Client, reqwest::Error> {
    let mut default_header = reqwest::header::HeaderMap::new();

    default_header.append(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_str(USER_AGENT).unwrap(),
    );

    default_header.append(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );

    Client::builder()
        .user_agent(USER_AGENT)
        .default_headers(default_header)
        .build()
}

pub fn fetch_pull_requests(
    client: &Client,
    query_body: QueryBody<crate::query::Variables>,
) -> Result<Response<crate::query::ResponseData>, reqwest::Error> {
    let res = client.post(GITHUB_API_URL).json(&query_body).send()?;

    let response_body: Response<crate::query::ResponseData> = res.json()?;

    Ok(response_body)
}

pub fn create_query_body(query: String, cursor: Option<String>) -> QueryBody<crate::query::Variables> {
    let variables = crate::query::Variables { query, cursor };

    Query::build_query(variables)
}
