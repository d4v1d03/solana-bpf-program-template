use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]

pub struct Mail {
    pub id: String,
    pub from_address: String,
    pub to_address: String,
    pub subject: String,
    pub body: String,
    pub sent_date: String,
  }

  pub struct MailAccount {
    pub inbox: Vec<Mail>,
    pub sent: Vec<Mail>,
  }