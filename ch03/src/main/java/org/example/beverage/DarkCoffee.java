package org.example.beverage;

public class DarkCoffee extends Beverage {

    public DarkCoffee(String description) {
        super(description);
    }

    @Override
    public Long getCost() {
        return 3500L;
    }
}
