#[cfg(test)]
mod test {
    use std::error::Error;

    use regex::Regex;

    use crate::rule::*;

    #[test]
	fn rules_tests() {
        let mut tweet = TweetInfo {
            text: "A77 B730 Symington - A78 Monkton - Closure, All lanes closed Northbound https://t.co/v42ucR1Q32 #TSIncident".to_string(),
            hour: 8,
            day: "Mon".to_string(),
            retweeted: false,
            user: 1,
            rtuser: 2,
            screen_name: &"trafficscotland".to_string(),
            followed_users: &vec![1u64, 2u64, 3u64],
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
        //it is the right time, the day, it should match
        assert!(rule.matches(&tweet));

        let eve_range = Range {
            start: 14,
            end: 18,
            excludes: Some(Regex::new("southbound|s/b").unwrap()),
        };

        rule.active_hours = Some(vec![eve_range]);

        //it is NOT the right time window, should not match
        assert!(!rule.matches(&tweet));

        let excl_range = Range {
            start: 6,
            end: 10,
            excludes: Some(Regex::new("northbound|n/b").unwrap()),
        };

        rule.active_hours = Some(vec![excl_range]);

        //it is the right time window, but it should exclude as exclude regex matches
        assert!(!rule.matches(&tweet));

        let none_range = Range {
            start: 6,
            end: 10,
            excludes: None,
        };

        rule.active_hours = Some(vec![none_range]);
        rule.active_days = Some(Regex::new("Tue|Wed|Fri").unwrap());

        //should not match, because tweet is on Monday and active days don't include monday
        assert!(!rule.matches(&tweet));

        rule.active_days = Some(Regex::new("Mon").unwrap());
        assert!(rule.matches(&tweet));
        //It is monday, should match again
        assert!(rule.matches(&tweet));

        tweet.retweeted = true;

        //should not match when one account we follow is RTing another
        assert!(!rule.matches(&tweet));

        tweet.rtuser = 5;

        //should match when an account RTs an account we DON'T follow
        assert!(rule.matches(&tweet));


        rule.excludes = Some(Regex::new("symington|monkton").unwrap());

        //excludes should exclude
        assert!(!rule.matches(&tweet));

        rule.excludes = None;
        rule.active_hours = None;
        rule.active_days = None;

        //should match when pm no restrictions
        assert!(rule.matches(&tweet));

    }
}