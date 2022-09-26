use crate::client::{BaseBind, ClientBind, Interface};
use serde::{Deserialize, Serialize};

pub mod apod {
    use super::*;

    #[derive(Debug, Deserialize, Serialize)]
    pub enum QueryParams {
        Date(String),
        StartDate(String),
        EndDate(String),
        Count(String),
        Thumbs(bool),
        Key(String),
    }

    pub struct Apod {}
    impl Interface for Apod {
        type Params = QueryParams;

        fn start(base: BaseBind) -> ClientBind {
            ClientBind::new(base, None, None)
        }

        fn query(&self, params: Self::Params) -> String {
            match params {
                QueryParams::Date(date) => format!("date={}", date),
                QueryParams::StartDate(start_date) => format!("start_date={}", start_date),
                QueryParams::EndDate(end_date) => format!("end_date={}", end_date),
                QueryParams::Count(count) => format!("count={}", count),
                QueryParams::Thumbs(thumbs) => format!("thumbs={}", thumbs),
                QueryParams::Key(key) => format!("api_key={}", key),
            }
        }
    }
}

pub mod neo {
    use super::*;

    #[derive(Debug, Deserialize, Serialize)]
    pub enum QueryParams {
        StartDate(String),
        EndDate(String),
        ApiKey(String),
    }

    pub struct Neo {}
    impl Interface for Neo {
        type Params = QueryParams;

        fn start(base: BaseBind) -> ClientBind {
            ClientBind::new(base, None, None)
        }

        fn query(&self, params: Self::Params) -> String {
            match params {
                QueryParams::StartDate(start_date) => format!("start_date={}", start_date),
                QueryParams::EndDate(end_date) => format!("end_date={}", end_date),
                QueryParams::ApiKey(api_key) => format!("api_key={}", api_key),
            }
        }
    }
}

pub mod donki {
    use super::*;

    pub mod cme {
        use super::*;

        #[derive(Debug, Deserialize, Serialize)]
        #[allow(non_snake_case)]
        pub enum QueryParams {
            StartDate(String),
            EndDate(String),
            MostAccurateOnly(bool),
            CompleteEntryOnly(bool),
            Speed(u64),
            HalfAngle(u64),
            Catalog(String),
            Keyword(String),
        }

        pub struct Cme {}
        impl Interface for Cme {
            type Params = QueryParams;

            fn start(base: BaseBind) -> ClientBind {
                ClientBind::new(base, None, None)
            }

            fn query(&self, params: Self::Params) -> String {
                match params {
                    QueryParams::StartDate(start_date) => format!("startDate={}", start_date),
                    QueryParams::EndDate(end_date) => format!("endDate={}", end_date),
                    QueryParams::MostAccurateOnly(most_accurate_only) => {
                        format!("mostAccurateOnly={}", most_accurate_only)
                    }
                    QueryParams::CompleteEntryOnly(complete_entry_only) => {
                        format!("completeEntryOnly={}", complete_entry_only)
                    }
                    QueryParams::Speed(speed) => format!("speed={}", speed),
                    QueryParams::HalfAngle(half_angle) => format!("halfAngle={}", half_angle),
                    QueryParams::Catalog(catalog) => format!("catalog={}", catalog),
                    QueryParams::Keyword(keyword) => format!("keyword={}", keyword),
                }
            }
        }
    }

    pub mod gst {
        use super::*;

        #[derive(Debug, Deserialize, Serialize)]
        #[allow(non_snake_case)]
        pub enum QueryParams {
            StartDate(String),
            EndDate(String),
        }

        pub struct Gst {}
        impl Interface for Gst {
            type Params = QueryParams;

            fn start(base: BaseBind) -> ClientBind {
                ClientBind::new(base, None, None)
            }

            fn query(&self, params: Self::Params) -> String {
                match params {
                    QueryParams::StartDate(start_date) => format!("startDate={}", start_date),
                    QueryParams::EndDate(end_date) => format!("endDate={}", end_date),
                }
            }
        }
    }

    pub mod ips {
        use super::*;

        #[derive(Debug, Deserialize, Serialize)]
        #[allow(non_snake_case)]
        pub enum QueryParams {
            StartDate(String),
            EndDate(String),
            Location(String),
            Catalog(String),
        }

        pub struct Ips {}
        impl Interface for Ips {
            type Params = QueryParams;

            fn start(base: BaseBind) -> ClientBind {
                ClientBind::new(base, None, None)
            }

            fn query(&self, params: Self::Params) -> String {
                match params {
                    QueryParams::StartDate(start_date) => format!("startDate={}", start_date),
                    QueryParams::EndDate(end_date) => format!("endDate={}", end_date),
                    QueryParams::Location(location) => format!("location={}", location),
                    QueryParams::Catalog(catalog) => format!("catalog={}", catalog),
                }
            }
        }
    }

    pub mod flr {
        use super::*;

        #[derive(Debug, Deserialize, Serialize)]
        #[allow(non_snake_case)]
        pub enum QueryParams {
            StartDate(String),
            EndDate(String),
        }

        pub struct Flr {}
        impl Interface for Flr {
            type Params = QueryParams;

            fn start(base: BaseBind) -> ClientBind {
                ClientBind::new(base, None, None)
            }

            fn query(&self, params: Self::Params) -> String {
                match params {
                    QueryParams::StartDate(start_date) => format!("startDate={}", start_date),
                    QueryParams::EndDate(end_date) => format!("endDate={}", end_date),
                }
            }
        }
    }
}

pub mod genelab {
    use super::*;

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]
    pub enum QueryParams {
        /// Comma separated list with mixture of single GLDS accession numbers and ranges
        GldsStudyIds(String),
        /// Current page number in pagination
        CurrPageNumber(u32),
        ResultsPerPage(u32),
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]
    pub enum Mode {
        File,
        Meta,
    }

    pub struct GeneLab {
        mode: Mode,
    }

    impl GeneLab {
        pub fn new(mode: Mode) -> Self {
            Self { mode }
        }

        pub fn make_base(&self) -> String {
            return match self.mode {
                Mode::File => {
                    String::from("https://genelab-data.ndc.nasa.gov/genelab/data/glds/file")
                }
                Mode::Meta => {
                    String::from("https://genelab-data.ndc.nasa.gov/genelab/data/glds/meta")
                }
            };
        }
    }

    impl Interface for GeneLab {
        type Params = QueryParams;

        fn start(base: BaseBind) -> ClientBind {
            ClientBind::new(base, None, None)
        }

        fn query(&self, params: Self::Params) -> String {
            let mut url = match self.mode {
                Mode::File => "file",
                Mode::Meta => "meta",
            }
            .to_string();

            match params {
                QueryParams::GldsStudyIds(study_ids) => {
                    url.push_str(&format!("?gldsStudyIds={}", study_ids))
                }
                QueryParams::CurrPageNumber(pg_number) => {
                    url.push_str(&format!("?currPageNumber={}", pg_number))
                }
                QueryParams::ResultsPerPage(results_per_page) => {
                    url.push_str(&format!("?resultsPerPage={}", results_per_page))
                }
            }

            return url;
        }
    }
}

pub mod insight {
    use super::*;

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]
    pub enum QueryParams {
        /// Version of this API
        Version(String),
        /// Default is JSON
        FeedType(String),
        /// API Key
        Key(String),
    }

    pub struct Insight {}
    impl Interface for Insight {
        type Params = QueryParams;

        fn start(base: BaseBind) -> ClientBind {
            ClientBind::new(base, None, None)
        }

        fn query(&self, params: Self::Params) -> String {
            match params {
                QueryParams::Version(version) => format!("version={}", version),
                QueryParams::FeedType(feed_type) => format!("feedType={}", feed_type),
                QueryParams::Key(key) => format!("key={}", key),
            }
        }
    }
}
