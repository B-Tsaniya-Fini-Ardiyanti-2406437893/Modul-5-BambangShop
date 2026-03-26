use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::model::subscriber::Subscriber;

// Singleton of Database
lazy_static! {
    pub static ref SUBSCRIBERS: DashMap<String, Subscriber> = DashMap::new();
}

pub struct SubscriberRepository;
impl SubscriberRepository {
    pub fn add(product_type: &str, subscriber: Subscriber) -> Subscriber {
        let subscriber_value = subscriber.clone();
        if SUBSCRIBERS.get(product_type).is_none() {
            SUBSCRIBERS.insert(product_type.to_string(), subscriber_value);
        } ;

        SUBSCRIBERS.get(product_type).unwrap()
            .insert(subscriber_value.url.clone(), subscriber_value);

        return subscriber;
    }

    pub fn list_all(product_type: &str) -> Vec<Subscriber> {
        if SUBSCRIBERS.get(product_type).is_none() {
            SUBSCRIBERS.insert(product_type.to_string(), subscriber_value);
        };

        return SUBSCRIBERS.get(product_type).unwrap().iter().map(|f| f.value().clone()).collect();
    }

    pub fn delete(product_type: &str, url: &str) -> Option<Subscriber> {
        if SUBSCRIBERS.get(product_type).is_none() {
            SUBSCRRIBERS.insert(String::from(product_type), DashMap::new());
        }

        let result = SUBSCRIBERS.get(product_type).unwrap();
        if !result.is_none() {
            return Some(result.unwrap().1);
        }
        return None;
    }
}