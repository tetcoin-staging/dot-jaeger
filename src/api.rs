// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of dot-jaeger.

// dot-jaeger is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// dot-jaeger is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with dot-jaeger.  If not, see <http://www.gnu.org/licenses/>.

use crate::cli::App;
use anyhow::Error;
use std::fmt;

pub const TRACES: &str = "/api/traces";

pub enum Endpoint {
    Traces,
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Endpoint::Traces => write!(f, "{}", TRACES),
        }
    }
}

pub struct JaegerApi<'a> {
    /// URL Where Jaeger Agent is running.
    /// Should be full URL including Port and protocol.
    /// # Example
    /// http://localhost:16686
    ///
    url: &'a str,
    service: &'a str,
}

impl<'a> JaegerApi<'a> {
    pub fn new(url: &'a str, service: &'a str) -> Self {
        Self { url, service }
    }

    pub fn traces(&self, app: &App) -> Result<String, Error> {
        let req = ureq::get(&endpoint(self.url, Endpoint::Traces));
        let req = ParamBuilder::new(self.service)
            .limit(app.limit)
            .pretty_print(app.pretty_print)
            .start(app.start)
            .end(app.end)
            .build(req);
        let response = req.call()?.into_string()?;
        Ok(response)
    }
}

fn endpoint(url: &str, endpoint: Endpoint) -> String {
    match endpoint {
        Endpoint::Traces => {
            format!("{}/{}", url, endpoint)
        }
    }
}

pub struct ParamBuilder<'a> {
    start: Option<usize>,
    end: Option<usize>,
    limit: Option<usize>,
    pretty_print: bool,
    service: &'a str,
}

impl<'a> ParamBuilder<'a> {
    pub fn new(service: &'a str) -> Self {
        Self {
            start: None,
            end: None,
            pretty_print: false,
            limit: None,
            service,
        }
    }

    pub fn start(mut self, start: Option<usize>) -> Self {
        self.start = start;
        self
    }

    pub fn end(mut self, end: Option<usize>) -> Self {
        self.end = end;
        self
    }

    pub fn limit(mut self, limit: Option<usize>) -> Self {
        self.limit = limit;
        self
    }

    pub fn pretty_print(mut self, pretty_print: bool) -> Self {
        self.pretty_print = pretty_print;
        self
    }

    pub fn build(self, req: ureq::Request) -> ureq::Request {
        let mut req = req
            .query("service", &self.service)
            .query("pretty_print", &self.pretty_print.to_string());

        if let Some(start) = self.start {
            req = req.query("start", &start.to_string());
        }
        if let Some(end) = self.end {
            req = req.query("end", &end.to_string());
        }
        if let Some(limit) = self.limit {
            req = req.query("limit", &limit.to_string());
        }

        req
    }
}
