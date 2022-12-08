package org.example.beverage;

public class HouseBlend extends Beverage {

    public HouseBlend(String description) {
        super(description);
    }

    @Override
    public Long getCost() {
        return 1500L;
    }
}
