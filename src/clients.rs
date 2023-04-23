/// Picture of the Day (APOD) client
pub mod apod;
/// Database of Notifications, Knowledge, Information (DONKI) clients
pub mod donki;
/// Earth Landsat Imagery Client
pub mod earth;
/// Earth Polychromatic Imaging Camera (EPIC) client
pub mod epic;
/// Near Earth Object (NEO) client
pub mod neo;

/// Re-exporting Subclients
pub mod prelude {
    pub use super::apod::Apod;
    pub use super::donki::cme::CME as Cme;
    pub use super::donki::flr::FLR as Flr;
    pub use super::donki::gst::GST as Gst;
    pub use super::donki::hss::HSS as Hss;
    pub use super::donki::ips::IPS as Ips;
    pub use super::donki::mpc::MPC as Mpc;
    pub use super::donki::rbe::RBE as Rbe;
    pub use super::donki::sep::SEP as Sep;
    pub use super::earth::Earth;
    pub use super::epic::epic_client::EPIC as Epic;
    pub use super::neo::feed::NeoFeed as NeoF;
    pub use super::neo::lookup::NeoLookup as NeoL;
}
