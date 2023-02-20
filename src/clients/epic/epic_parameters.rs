use crate::core::Params;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EPICParams<'p>
{
    /// Metadata on the most recent date of natural color imagery
    Natural(&'p str),
    /// Metadata for natural color imagery available for a given date
    NaturalDate(&'p str),
    /// A listing of all dates with available natural color imagery
    NaturalAll,
    /// Alternate listing of all dates with available color imagery
    NaturalAvailable,
    /// Metadata on the most recent date of enhanced color imagery
    Enhanced,
    /// Metadata for enhanced color imagery available for a given date
    EhancedDate(&'p str),
    /// A listing of all dates with available enhanced color imagery
    EnhancedAll,
    /// Alternate listing of all dates with available enhanced color imagery
    EnhancedAvailable,
}

impl<'p> Default for EPICParams<'p>
{
    fn default() -> Self
    {
        EPICParams::NaturalAll
    }
}

impl<'p> Into<String> for EPICParams<'p>
{
    fn into(self) -> String
    {
        match self
        {
            EPICParams::Natural(date) => format!("natural/date/{}", date),
            EPICParams::NaturalDate(date) => format!("natural/date/{}", date),
            EPICParams::NaturalAll => "natural/all".to_string(),
            EPICParams::NaturalAvailable => "natural/available".to_string(),
            EPICParams::Enhanced => "enhanced".to_string(),
            EPICParams::EhancedDate(date) => format!("enhanced/date/{}", date),
            EPICParams::EnhancedAll => "enhanced/all".to_string(),
            EPICParams::EnhancedAvailable => "enhanced/available".to_string(),
        }
    }
}

impl<'p> Params for EPICParams<'p> {}
