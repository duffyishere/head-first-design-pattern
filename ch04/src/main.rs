fn main() {
    let pizza_store: NY_pizza_style_store = NY_pizza_style_store {};
    let pizza: Pizza = pizza_store.order_pizza(String::from("veggie"));
    println!("{:?}", pizza);
}

#[derive(Debug)]
pub struct Pizza {
    name:String,
    dough:String,
    sauce:String,
    toppings:Vec<String>
}

impl Pizza {
    fn prepare(&self) {
        println!("{} 피자를 준비 중입니다.", self.name);
        println!("{}를 펼칩니다.", self.dough);
        println!("{} 소스를 올립니다.", self.sauce);
        self.toppings.iter().for_each(|topping| {
            println!("{}를 올립니다.", topping);
        });
    }

    fn bake(&self) {
        println!("{} 피자를 굽는 중입니다.", self.name);
    }

    fn cut(&self) {
        println!("{} 피자를 자르는 중입니다.", self.name);
    }

    fn packaging(&self) {
        println!("{} 피자를 포장하는 중입니다.", self.name);
    }
}

pub trait NY_style_cheese_pizza {
    fn new() -> Pizza;
}

pub trait NY_style_clam_pizza {
    fn new() -> Pizza;
}

pub trait NY_style_pepperoni_pizza {
    fn new() -> Pizza;
}

pub trait NY_style_veggie_pizza {
    fn new() -> Pizza;
}

pub trait Chicago_style_cheese_pizza {
    fn new() -> Pizza;
}

pub trait Chicago_style_clam_pizza {
    fn new() -> Pizza;
}

pub trait Chicago_style_pepperoni_pizza {
    fn new() -> Pizza;
}

pub trait Chicago_style_veggie_pizza {
    fn new() -> Pizza;
}

impl NY_style_cheese_pizza for Pizza {
    fn new() -> Pizza {
        Pizza {
            name: String::from("뉴욕 스타일 치즈 피자"),
            dough: String::from("얇은 크러스트"),
            sauce: String::from("마리나라 소스"),
            toppings: vec![String::from("모짜렐라 치즈")]
        }
    }
}

impl NY_style_clam_pizza for Pizza {
    fn new() -> Pizza {
        Pizza {
            name: String::from("뉴욕 스타일 조개 피자"),
            dough: String::from("얇은 크러스트"),
            sauce: String::from("마리나라 소스"),
            toppings: vec![String::from("모짜렐라 치즈"), String::from("후라이드 조개")]
        }
    }
}

impl NY_style_pepperoni_pizza for Pizza {
    fn new() -> Pizza {
        Pizza {
            name: String::from("뉴욕 스타일 페퍼로니 피자"),
            dough: String::from("얇은 크러스트"),
            sauce: String::from("마리나라 소스"),
            toppings: vec![String::from("모짜렐라 치즈"), String::from("페퍼로니")]
        }
    }
}

impl NY_style_veggie_pizza for Pizza {
    fn new() -> Pizza {
        Pizza {
            name: String::from("뉴욕 스타일 채소 피자"),
            dough: String::from("얇은 크러스트"),
            sauce: String::from("마리나라 소스"),
            toppings: vec![String::from("모짜렐라 치즈"), String::from("피망"), String::from("양상추"), String::from("양파")]
        }
    }
}

impl Chicago_style_cheese_pizza for Pizza {
    fn new() -> Pizza {
        Pizza {
            name: String::from("시카고 스타일 치즈 피자"),
            dough: String::from("크러스트"),
            sauce: String::from("플랫 소스"),
            toppings: vec![String::from("모짜렐라 치즈")]
        }
    }
}

impl Chicago_style_clam_pizza for Pizza {
    fn new() -> Pizza {
        Pizza {
            name: String::from("시카고 스타일 조개 피자"),
            dough: String::from("크러스트"),
            sauce: String::from("플랫 소스"),
            toppings: vec![String::from("모짜렐라 치즈"), String::from("후라이드 조개")]
        }
    }
}

impl Chicago_style_pepperoni_pizza for Pizza {
    fn new() -> Pizza {
        Pizza {
            name: String::from("시카고 스타일 페퍼로니 피자"),
            dough: String::from("크러스트"),
            sauce: String::from("플랫 소스"),
            toppings: vec![String::from("모짜렐라 치즈"), String::from("페퍼로니")]
        }
    }
}

impl Chicago_style_veggie_pizza for Pizza {
    fn new() -> Pizza {
        Pizza {
            name: String::from("시카고 스타일 채소 피자"),
            dough: String::from("크러스트"),
            sauce: String::from("플랫 소스"),
            toppings: vec![String::from("모짜렐라 치즈"), String::from("피망"), String::from("양상추"), String::from("양파")]
        }
    }
}

struct Chicago_pizza_style_store;

struct NY_pizza_style_store;

pub trait Pizza_store {
    fn order_pizza(&self, pizza_type: String) -> Pizza {
        let pizza = self.create_pizza(pizza_type);
        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.packaging();

        pizza
    }

    fn create_pizza(&self, pizza_type: String) -> Pizza;
}

impl Pizza_store for Chicago_pizza_style_store {
    fn create_pizza(&self, pizza_type: String) -> Pizza {
        let pizza = match pizza_type.as_str() {
            "cheese" => <Pizza as Chicago_style_cheese_pizza>::new(),
            "clam" => <Pizza as Chicago_style_clam_pizza>::new(),
            "pepperoni" => <Pizza as Chicago_style_pepperoni_pizza>::new(),
            "veggie" => <Pizza as Chicago_style_veggie_pizza>::new(),
            _ => panic!("{} 피자는 없습니다.", pizza_type)
        };

        pizza
    }
}

impl Pizza_store for NY_pizza_style_store {
    fn create_pizza(&self, pizza_type: String) -> Pizza {
        let pizza = match pizza_type.as_str() {
            "cheese" => <Pizza as NY_style_cheese_pizza>::new() ,
            "clam" => <Pizza as NY_style_clam_pizza>::new(),
            "pepperoni" => <Pizza as NY_style_pepperoni_pizza>::new(),
            "veggie" => <Pizza as NY_style_veggie_pizza>::new(),
            _ => panic!("{} 피자는 없습니다.", pizza_type)
        };

        pizza
    }
}
