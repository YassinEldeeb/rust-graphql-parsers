use criterion::{black_box, criterion_group, criterion_main, Criterion};
use graphql_query::ast::ParseNode;
use std::fs;

pub fn graphql_parsers(c: &mut Criterion) {
    // Load the query and schema files
    let schema_dir = "fixtures/schemas/";
    let query_dir = "fixtures/queries/";

    let schema_files: Vec<_> = fs::read_dir(schema_dir)
        .unwrap()
        .map(|res| res.unwrap().path())
        .collect();

    let query_files: Vec<_> = fs::read_dir(query_dir)
        .unwrap()
        .map(|res| res.unwrap().path())
        .collect();

    // // Benchmark async-graphql-parser for each schema file
    // for schema_file in &schema_files {
    //     let schema_name = schema_file.file_name().unwrap().to_str().unwrap();
    //     c.bench_function(
    //         &format!("async-graphql-parser schema parsing / {}", schema_name),
    //         |b| {
    //             b.iter(|| {
    //                 let schema_input = fs::read_to_string(schema_file).unwrap();
    //                 let parsed_schema = async_graphql_parser::parse_schema(&schema_input);
    //                 let _ = black_box(parsed_schema);
    //             })
    //         },
    //     );
    // }

    // Benchmark async-graphql-parser for each query file
    for query_file in &query_files {
        let query_name = query_file.file_name().unwrap().to_str().unwrap();
        c.bench_function(&format!("asyync-graphql-parser / {}", query_name), |b| {
            b.iter(|| {
                let query_input = fs::read_to_string(query_file).unwrap();
                let parsed_query = async_graphql_parser::parse_query(&query_input);
                let _ = black_box(parsed_query);
            })
        });
    }

    // Benchmark graphql_query (Stellate) for each query file
    for query_file in &query_files {
        let query_name = query_file.file_name().unwrap().to_str().unwrap();
        c.bench_function(&format!("graphql_query (Stellate) / {}", query_name), |b| {
            b.iter(|| {
                let query_input = fs::read_to_string(query_file).unwrap();
                let ctx = graphql_query::ast::ASTContext::new();
                let parsed_query = graphql_query::ast::Document::parse(&ctx, query_input).unwrap();

                let _ = black_box(parsed_query);
            })
        });
    }

    for query_file in &query_files {
        let query_name = query_file.file_name().unwrap().to_str().unwrap();
        c.bench_function(&format!("graphql-parser / {}", query_name), |b| {
            b.iter(|| {
                let query_input = fs::read_to_string(query_file).unwrap();
                let parsed_query = graphql_parser::query::parse_query::<&str>(&query_input);
                let _ = black_box(parsed_query);
            })
        });
    }
}

criterion_group!(benches, graphql_parsers);
criterion_main!(benches);
