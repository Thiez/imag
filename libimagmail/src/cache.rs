use mail::Mail;

pub struct MessageId(String);

pub struct MailCache {
    hm: HashMap<MessageId, Mail>,
}

impl MailCache {

    pub fn new() -> MailCache {
        MailCache { hm: HashMap::new() }
    }

    /// Create a MailCache from a Iterator<Mail>
    pub fn from_iter<I: Iterator<Mail>>(i: I) -> Result<MailCache> {
        unimplemented!()
    }

    /// Update all `Mail` objects which are cached internally.
    pub fn update(&mut self, store: &Store) -> Result<()> {
        unimplemented!()
    }

    /// Create links between mails based on message-ids
    pub fn create_links(&mut self) -> Result<()> {
        unimplemented!()
    }

}

