package org.example.condimentDecorator;

import org.example.beverage.Beverage;

public class Mocha extends CondimentDecorator {

    public Mocha(Beverage beverage) {
        super(beverage);
    }

    @Override
    public Long getCost() {
        return this.getBeverage().getCost() + 1000L;
    }

    @Override
    public String getDescription() {
        return this.getBeverage().getDescription()+"(모카)";
    }
}
