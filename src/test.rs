#[cfg(test)]
mod test {
    use std::error::Error;

    use regex::Regex;

    use crate::rule::*;

    #[test]
	fn rules_timing_tests() {
        let tweet = TweetInfo {
            text: "A77 B730 Symington - A78 Monkton - Closure, All lanes closed Northbound https://t.co/v42ucR1Q32 #TSIncident".to_string(),
            hour: 8,
            day: "Mon".to_string(),
            retweeted: false,
            user: 1,
            screen_name: &"trafficscotland".to_string(),
            followed_users: &vec![1u64],
        };

        let range = Range {
            start: 6,
            end: 10,
            excludes: Some(Regex::new("southbound|s/b").unwrap()),
        };

        let chat = Chat {
            chat: 123,
        };

        let mut rule = Rule {
            name: "trafficscotland".to_string(),
            chats: vec![chat],
            includes: Regex::new("a76[\\D$]|irvine|kilmarnock|a77[\\D$]|m77[\\D$]|bellfield|galston").unwrap(),
            excludes: Some(Regex::new("safety|careful").unwrap()),
            active_hours: Some(vec![range]),
            active_days: Some(Regex::new("Mon|Tue|Wed|Thu|Fri").unwrap()),
        };

        assert!(rule.matches(&tweet));

        let eve_range = Range {
            start: 14,
            end: 18,
            excludes: Some(Regex::new("southbound|s/b").unwrap()),
        };

        rule.active_hours = Some(vec![eve_range]);

        assert!(!rule.matches(&tweet));

        let excl_range = Range {
            start: 6,
            end: 10,
            excludes: Some(Regex::new("northbound|n/b").unwrap()),
        };

        rule.active_hours = Some(vec![excl_range]);

        assert!(!rule.matches(&tweet));

    }
}