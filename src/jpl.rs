use std::error::Error;

use crate::pretty::*;

/// Atmospheric Impact Data
#[derive(Debug, PartialEq)]
pub struct FireballClient {
    base_url: String,
    limit: Option<u32>,
}

impl FireballClient {
    pub fn new() -> Self {
        FireballClient {
            base_url: String::from("https://ssd-api.jpl.nasa.gov/fireball.api"),
            limit: None,
        }
    }

    pub fn limit(&mut self, limit: u32) {
        self.limit = Some(limit)
    }

    pub fn query(&self) -> Result<String, Box<dyn Error>> {
        if self.limit.is_none() {
            let url = format!("{}", self.base_url);

            let res: String = ureq::get(&url).call()?.into_string()?;
            let fireball = to_string_pretty(res).unwrap();
            Ok(fireball)
        } else {
            let limit = self.limit.as_ref().unwrap();

            let url = format!("{}?limit={}", self.base_url, limit);

            let res: String = ureq::get(&url).call()?.into_string()?;
            let fireball = to_string_pretty(res).unwrap();

            Ok(fireball)
        }
    }
}

/// Base Client for the JPL Mission Design API in Query Mode.
/// # Example
/// ```
/// use voyager_client::jpl::*;
///
/// let mut base = MissionDesign::new();
///
/// base.query(QueryType::DES, "2012%20TC4").unwrap();
/// ```
///

/// Mission Design in Q Mode (query)
#[derive(Debug, PartialEq)]
pub struct MissionDesign {
    base_url: String,
}

#[derive(Debug, PartialEq)]
pub enum QueryType {
    /// designation (provisional or IAU-number) of the desired object (e.g., 2015 AB or 141P or 433).
    /// NOTE: when submitting a des containing a space in your query string, you must replace the space with %20, for example 2015%20AB.
    DES,
    /// object search string: designation in various forms (including MPC packed form), case-insensitive name, or SPK-ID;
    /// designation can be an alternate provisional designation; examples: atira, 2003 CP20, 2003cp20, K03C20P, 163693, 2163693
    SSTR,
}

impl MissionDesign {
    pub fn new() -> Self {
        MissionDesign {
            base_url: String::from("https://ssd-api.jpl.nasa.gov/mdesign.api?"),
        }
    }

    /// Mission Design: Q mode (query)
    pub fn query(&self, query_type: QueryType, query: &str) -> Result<String, Box<dyn Error>> {
        // Default Query Mode
        match query_type {
            QueryType::DES => {
                let url = format!("{}des={}", self.base_url, query);

                let res: String = ureq::get(&url).call()?.into_string()?;
                let mission = to_string_pretty(res).unwrap();

                Ok(mission)
            }
            QueryType::SSTR => {
                let url = format!("{}sstr={}", self.base_url, query);

                let res: String = ureq::get(&url).call()?.into_string()?;
                let mission = to_string_pretty(res).unwrap();

                Ok(mission)
            }
        }
    }
}

/// Base Client for Mission Design in Accessible Mode (A)
/// # Example
/// ```
/// use voyager_client::jpl::*;
///
/// let mut base = MissionDesignAccessible::new();
/// base.limit(10);
/// base.crit(1);
/// base.year(String::from("2025,2026,2027,2028,2029"));
///
/// base.lim_crit_year().unwrap();
/// ```
#[derive(Debug, PartialEq)]
pub struct MissionDesignAccessible {
    base_url: String,
    limit: Option<u32>,
    crit: Option<u8>,
    year: Option<String>,
    rdvz: Option<bool>,
    class: Option<String>,
}

impl MissionDesignAccessible {
    /// Create a new MissionDesignAccessible base client with None set for the limit, crit, year, rdvz and class fields
    pub fn new() -> Self {
        MissionDesignAccessible {
            base_url: String::from("https://ssd-api.jpl.nasa.gov/mdesign.api?"),
            limit: None,
            crit: None,
            year: None,
            rdvz: None,
            class: None,
        }
    }

    pub fn limit(&mut self, limit: u32) {
        self.limit = Some(limit)
    }

    pub fn crit(&mut self, crit: u8) {
        self.crit = Some(crit)
    }

    pub fn year(&mut self, year: String) {
        self.year = Some(year)
    }

    pub fn rdvz(&mut self, rdvz: bool) {
        self.rdvz = Some(rdvz)
    }

    pub fn class(&mut self, class: String) {
        self.class = Some(class)
    }

    /// Must set Limit, Crit, and year values
    pub fn lim_crit_year(&self) -> Result<String, Box<dyn Error>> {
        assert!(self.limit != None, "Limit is None");
        assert!(self.crit != None, "Crit is None");
        assert!(self.year != None, "Year is None");

        let url = format!(
            "{}lim={}&crit={}&year={}",
            self.base_url,
            self.limit.as_ref().unwrap(),
            self.crit.as_ref().unwrap(),
            self.year.as_ref().unwrap()
        );

        let res = ureq::get(&url).call()?.into_string()?;
        let pretty = to_string_pretty(res).unwrap();

        Ok(pretty)
    }
}

/// Base Client for Mission Design in Map Mode (M)
/// # Example
/// ```
/// use voyager_client::jpl::*;
///
/// let mut base = MissionDesignMap::new();
/// base.designation("2012%20TC4");
/// base.mjd(58490);
/// base.span(3652);
/// base.tof(10, 36);
/// base.step(2);
///
/// base.query().unwrap();
/// ```

#[derive(Debug, PartialEq)]
pub struct MissionDesignMap {
    base_url: String,
    des: Option<String>,
    mjd0: Option<u32>,
    span: Option<u32>,
    tof_min: Option<u32>,
    tof_max: Option<u32>,
    step: Option<u8>,
}

impl MissionDesignMap {
    pub fn new() -> Self {
        MissionDesignMap {
            base_url: String::from("https://ssd-api.jpl.nasa.gov/mdesign.api?"),
            des: None,
            mjd0: None,
            span: None,
            tof_min: None,
            tof_max: None,
            step: None,
        }
    }

    pub fn designation(&mut self, des: &str) {
        self.des = Some(String::from(des))
    }

    pub fn mjd(&mut self, x: u32) {
        self.mjd0 = Some(x)
    }

    pub fn span(&mut self, span: u32) {
        self.span = Some(span)
    }

    pub fn tof(&mut self, min: u32, max: u32) {
        self.tof_min = Some(min);
        self.tof_max = Some(max);
    }

    pub fn step(&mut self, step: u8) {
        self.step = Some(step)
    }

    pub fn query(&self) -> Result<String, Box<dyn Error>> {
        assert!(self.des != None, "Des is None");
        assert!(self.mjd0 != None, "Mjd0 is None");
        assert!(self.span != None, "Span is None");
        assert!(self.tof_min != None, "tof_min is None");
        assert!(self.tof_max != None, "tof_max is None");
        assert!(self.step != None, "Step is None");

        let url = format!(
            "{}des={}&mjd0={}&span={}&tof-min={}&tof-max={}&step={}",
            self.base_url,
            self.des.as_ref().unwrap(),
            self.mjd0.as_ref().unwrap(),
            self.span.as_ref().unwrap(),
            self.tof_min.as_ref().unwrap(),
            self.tof_max.as_ref().unwrap(),
            self.step.as_ref().unwrap()
        );

        println!("Url: {}", url);

        let res: String = ureq::get(&url).call()?.into_string()?;
        let map = to_string_pretty(res).unwrap();

        Ok(map)
    }
}