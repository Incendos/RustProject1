use std::collections::HashMap;
use crate::network::RatesResponse;

pub struct Rates {
    rates: HashMap<String, f32>,
    base: String,
    date: String,
}

impl From<RatesResponse> for Rates {
    fn from(rates: RatesResponse) -> Self {
        Rates {
            rates: rates.rates().clone(),
            base: rates.base().clone(),
            date: rates.date().clone(),
        }
    }
}
impl Rates {
    pub fn print(&self) {
        println!("base: {}", self.base);
        println!("date: {}", self.date);
        for (k, v) in &self.rates {
            println!("{}: {}", k, v);
        }
    }

    pub fn convert(&self, from: &str, to: &str, amount: f32) -> Result<f32, String> {
        let from_r = self.rates.get(from);
        if from_r.is_none() {
            return Err(format!("Could not find '{}' in rates", from));
        }

        let to_r = self.rates.get(to);
        if to_r.is_none() {
            return Err(format!("Could not find '{}' in rates", to));
        }

        Ok((amount / from_r.unwrap()) * to_r.unwrap())
    }
    pub fn rates(&self) -> &HashMap<String, f32> {
        &self.rates
    }

    pub fn date(&self) -> &String {
        &self.date
    }

    pub fn with_base(&self, base: &str) -> Result<Rates, String> {
        if !self.rates.contains_key(base) {
            return Err(format!("Could not find currency '{}'", base))
        }

        let mut new_rates = HashMap::new();

        for (k, v) in &self.rates {
            new_rates.insert(k.clone(), v.clone() * (1.0f32 / self.rates.get(base).unwrap()));
        }

        Ok(Rates {
            base: base.to_string(),
            date: self.date.clone(),
            rates: new_rates,
        })
    }
}