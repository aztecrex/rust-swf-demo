

pub type Dollars = i64;
pub type Timestamp = i64;
pub type AccountId = u128;

#[derive(Clone)]
pub struct Charge(String, Dollars);

#[derive(Clone)]
pub struct Entry(String, Dollars);

pub type Have<T> = Option<T>; // things we have yet to receive or compute
pub type Prev<T> = Option<T>; // something from previous or nothing if new account

#[derive(Clone)]
pub struct Invoice {
    pub when: Timestamp,
    pub charges: Vec<Charge>,
    pub balance: Dollars,
    pub previous_balance: Dollars,
    pub activity: Vec<Entry>,
}

#[derive(Clone)]
pub struct InvoiceFlow {
    account_id: AccountId,
    invoice_for: Timestamp,
    last_invoice_for: Have<Prev<Timestamp>>,
    charges: Have<Vec<Charge>>,
    previous_balance: Have<Prev<Dollars>>,
    activity: Have<Vec<Entry>>,
    invoice: Have<Invoice>,
}


pub trait Transducer: Sized {
    type Ev;
    type Eff;
    fn accept(self, event: &Self::Ev) -> (Vec<Self::Eff>, Self);
}

pub enum InvoiceEffect {
    FetchLastInvoice(AccountId),
    FetchUninvoicedCharges(AccountId),
}

pub enum InvoiceEvent {
    Init,
    LastInvoice(Prev<Invoice>),
    UninvoicedCharges(Vec<Charge>),
}

impl Transducer for InvoiceFlow {
    type Ev = InvoiceEvent;
    type Eff = InvoiceEffect;
    fn accept(self, event: &Self::Ev) -> (Vec<Self::Eff>, Self) {
        use self::InvoiceEvent::*;
        match event {
            Init => init(self),
            LastInvoice(invoice) => update_invoice(self, invoice),
            UninvoicedCharges(charges) => update_charges(self, charges),
        }
    }
}

type Next = (Vec<InvoiceEffect>, InvoiceFlow);

fn update_invoice(model: InvoiceFlow, invoice_if_existing: &Prev<Invoice>) -> Next {
    let mi = invoice_if_existing.as_ref();
    let next = InvoiceFlow {
        last_invoice_for: Some(mi.map(|inv| inv.when)),
        previous_balance: Some(mi.map(|inv| inv.balance)),
        ..model
    };
    let effects = vec!();
    (effects, next)
}

fn update_charges(model: InvoiceFlow, charges: &Vec<Charge>) -> Next {
    let next =  InvoiceFlow {
        charges: Some(charges.clone()),
        ..model
    };
    let effects = vec!();
    (effects, next)
}


fn init(model: InvoiceFlow) -> Next {
    (
        vec!(
            InvoiceEffect::FetchLastInvoice(model.account_id),
            InvoiceEffect::FetchUninvoicedCharges(model.account_id),
        ),
        model
    )
}

