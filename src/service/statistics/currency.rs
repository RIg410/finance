use crate::dao::model::currency::{Currency, CurrencyRate};
use crate::service::decimal::Decimal;
use chrono::{Duration, NaiveDateTime};

pub struct CurrencyStatistic {
    id: i64,
    ticker: String,
    rates: Vec<CurrencyRate>,
}

impl CurrencyStatistic {
    pub fn new(curr: Currency, mut rates: Vec<CurrencyRate>) -> Self {
        if rates.iter().any(|r| r.currency_id != curr.id) {
            panic!("Invalid currency rates");
        }

        rates.sort_by(|a, b| a.date.cmp(&b.date));
        Self {
            id: curr.id,
            ticker: curr.name,
            rates,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn ticker(&self) -> &String {
        &self.ticker
    }

    pub fn get_rate(&self, date: NaiveDateTime) -> Option<Decimal> {
        let mut rate = None;
        for r in &self.rates {
            if r.date <= date {
                rate = Some(r.rate);
            } else {
                break;
            }
        }
        rate
    }

    pub fn get_points(&self, start_date: NaiveDateTime, end_date: NaiveDateTime) -> Vec<Point> {
        self.rates
            .iter()
            .filter(|r| r.date >= start_date && r.date <= end_date)
            .map(|r| Point {
                date: r.date,
                value: r.rate,
            })
            .collect()
    }

    pub fn build_graph(
        &self,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
        point_size: Duration,
    ) -> CurrencyGraph {
        let mut points = Vec::new();
        let mut date = start_date;
        while date <= end_date {
            let segment_end = date + point_size;
            let point = average_rate(&self.get_points(date, segment_end));
            if point == Decimal::int(0) {
                if let Some(r) = self.get_rate(date) {
                    points.push(Point { date, value: r })
                }
            } else {
                points.push(Point { date, value: point });
            }
            date = segment_end;
        }

        CurrencyGraph {
            ticker: self.ticker.clone(),
            points,
        }
    }
}

fn average_rate(points: &[Point]) -> Decimal {
    if points.is_empty() {
        return Decimal::int(0);
    }

    let mut sum = Decimal::int(0);
    for p in points {
        sum += p.value;
    }
    sum / Decimal::int(points.len() as i64)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub date: NaiveDateTime,
    pub value: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrencyGraph {
    pub ticker: String,
    pub points: Vec<Point>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::decimal::dec;

    fn make_usd_currency_statistic() -> CurrencyStatistic {
        CurrencyStatistic::new(
            Currency {
                id: 0,
                name: "USD".to_string(),
                ticker: "USD".to_string(),
            },
            vec![
                CurrencyRate {
                    id: 0,
                    currency_id: 0,
                    rate: dec("1"),
                    date: time("2020-01-01 00:00:00"),
                },
                CurrencyRate {
                    id: 0,
                    currency_id: 0,
                    rate: dec("2"),
                    date: time("2020-01-02 00:00:00"),
                },
                CurrencyRate {
                    id: 0,
                    currency_id: 0,
                    rate: dec("3"),
                    date: time("2020-01-03 00:00:00"),
                },
                CurrencyRate {
                    id: 0,
                    currency_id: 0,
                    rate: dec("4"),
                    date: time("2020-01-04 00:00:00"),
                },
                CurrencyRate {
                    id: 0,
                    currency_id: 0,
                    rate: dec("5"),
                    date: time("2020-01-05 00:00:00"),
                },
                CurrencyRate {
                    id: 0,
                    currency_id: 0,
                    rate: dec("6"),
                    date: time("2020-01-06 00:00:00"),
                },
                CurrencyRate {
                    id: 0,
                    currency_id: 0,
                    rate: dec("7"),
                    date: time("2020-01-07 00:00:00"),
                },
                CurrencyRate {
                    id: 0,
                    currency_id: 0,
                    rate: dec("8"),
                    date: time("2020-01-08 00:00:00"),
                },
                CurrencyRate {
                    id: 0,
                    currency_id: 0,
                    rate: dec("9"),
                    date: time("2020-01-09 00:00:00"),
                },
                CurrencyRate {
                    id: 0,
                    currency_id: 0,
                    rate: dec("10"),
                    date: time("2020-01-10 00:00:00"),
                },
            ],
        )
    }

    #[test]
    fn test_build_graph() {
        let stat = make_usd_currency_statistic();
        assert_eq!(
            stat.build_graph(
                time("2020-01-01 00:00:00"),
                time("2020-01-01 00:00:00"),
                Duration::hours(1)
            ),
            CurrencyGraph {
                ticker: "USD".to_string(),
                points: vec![Point {
                    date: time("2020-01-01 00:00:00"),
                    value: dec("1")
                }]
            }
        );

        assert_eq!(
            stat.build_graph(
                time("2020-01-01 00:00:00"),
                time("2020-01-02 00:00:00"),
                Duration::hours(1)
            ),
            CurrencyGraph {
                ticker: "USD".to_string(),
                points: vec![
                    Point {
                        date: time("2020-01-01 00:00:00"),
                        value: dec("1.0")
                    },
                    Point {
                        date: time("2020-01-01 01:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 02:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 03:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 04:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 05:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 06:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 07:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 08:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 09:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 10:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 11:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 12:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 13:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 14:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 15:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 16:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 17:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 18:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 19:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 20:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 21:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 22:00:00"),
                        value: dec("1.0"),
                    },
                    Point {
                        date: time("2020-01-01 23:00:00"),
                        value: dec("2"),
                    },
                    Point {
                        date: time("2020-01-02 00:00:00"),
                        value: dec("2"),
                    }
                ]
            }
        );
    }

    #[test]
    fn test_get_points() {
        let stat = make_usd_currency_statistic();
        assert_eq!(
            stat.get_points(time("2019-01-01 00:00:00"), time("2019-12-20 00:00:00")),
            Vec::<Point>::new()
        );

        assert_eq!(
            stat.get_points(time("2020-01-01 00:00:00"), time("2020-01-01 00:00:00")),
            vec![Point {
                date: time("2020-01-01 00:00:00"),
                value: dec("1")
            }]
        );

        assert_eq!(
            stat.get_points(time("2020-01-01 00:00:00"), time("2020-01-02 00:00:00")),
            vec![
                Point {
                    date: time("2020-01-01 00:00:00"),
                    value: dec("1")
                },
                Point {
                    date: time("2020-01-02 00:00:00"),
                    value: dec("2")
                }
            ]
        );

        assert_eq!(
            stat.get_points(time("2020-01-01 00:00:00"), time("2020-01-03 00:00:00")),
            vec![
                Point {
                    date: time("2020-01-01 00:00:00"),
                    value: dec("1")
                },
                Point {
                    date: time("2020-01-02 00:00:00"),
                    value: dec("2")
                },
                Point {
                    date: time("2020-01-03 00:00:00"),
                    value: dec("3")
                }
            ]
        );

        assert_eq!(
            stat.get_points(time("2020-01-01 00:00:00"), time("2020-01-04 00:00:00")),
            vec![
                Point {
                    date: time("2020-01-01 00:00:00"),
                    value: dec("1")
                },
                Point {
                    date: time("2020-01-02 00:00:00"),
                    value: dec("2")
                },
                Point {
                    date: time("2020-01-03 00:00:00"),
                    value: dec("3")
                },
                Point {
                    date: time("2020-01-04 00:00:00"),
                    value: dec("4")
                }
            ]
        );
        assert_eq!(
            stat.get_points(time("2020-01-10 00:00:00"), time("2020-01-20 00:00:00")),
            vec![Point {
                date: time("2020-01-10 00:00:00"),
                value: dec("10")
            }]
        );
        assert_eq!(
            stat.get_points(time("2020-01-11 00:00:00"), time("2020-01-20 00:00:00")),
            Vec::<Point>::new()
        );
    }

    #[test]
    fn test_rete_on_date() {
        let stat = make_usd_currency_statistic();
        assert_eq!(stat.get_rate(time("2019-01-01 00:00:00")), None);
        assert_eq!(
            stat.get_rate(time("2020-01-01 00:00:00")),
            Some(Decimal::int(1))
        );
        assert_eq!(
            stat.get_rate(time("2020-01-02 00:00:00")),
            Some(Decimal::int(2))
        );
        assert_eq!(
            stat.get_rate(time("2020-01-03 00:00:00")),
            Some(Decimal::int(3))
        );
        assert_eq!(
            stat.get_rate(time("2020-01-04 00:00:00")),
            Some(Decimal::int(4))
        );
        assert_eq!(
            stat.get_rate(time("2020-01-04 23:59:59")),
            Some(Decimal::int(4))
        );
        assert_eq!(
            stat.get_rate(time("2020-01-05 00:00:00")),
            Some(Decimal::int(5))
        );
        assert_eq!(
            stat.get_rate(time("2020-01-11 00:00:00")),
            Some(Decimal::int(10))
        );
    }

    #[test]
    fn test_average_rate() {
        let points = vec![
            Point {
                date: time("2020-01-01 00:00:00"),
                value: Decimal::int(1),
            },
            Point {
                date: time("2020-01-02 00:00:00"),
                value: Decimal::int(2),
            },
            Point {
                date: time("2020-01-03 00:00:00"),
                value: Decimal::int(3),
            },
        ];
        assert_eq!(average_rate(&points), Decimal::int(2));
    }

    #[test]
    fn test_average_rate_empty() {
        let points = Vec::new();
        assert_eq!(average_rate(&points), Decimal::int(0));
    }

    #[test]
    fn test_average_rate_one() {
        let points = vec![Point {
            date: time("2020-01-01 00:00:00"),
            value: Decimal::int(1),
        }];
        assert_eq!(average_rate(&points), Decimal::int(1));
    }

    #[test]
    fn test_average_rate_two() {
        let points = vec![
            Point {
                date: time("2020-01-01 00:00:00"),
                value: dec("1"),
            },
            Point {
                date: time("2020-01-02 00:00:00"),
                value: dec("2"),
            },
        ];
        assert_eq!(average_rate(&points), dec("1.50"));
    }

    fn time(val: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(val, "%Y-%m-%d %H:%M:%S").unwrap()
    }
}
