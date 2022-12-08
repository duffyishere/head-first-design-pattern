package org.example.condimentDecorator;

import org.example.beverage.Beverage;

public abstract class CondimentDecorator extends Beverage {

    private Beverage beverage;

    public CondimentDecorator(Beverage beverage) {
        super(beverage.getDescription(), beverage.getSize());
        this.beverage = beverage;
    }

    public abstract String getDescription();

    public Beverage getBeverage() {
        return beverage;
    }
}
