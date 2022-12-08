package org.example.beverage;

import org.example.size.BeverageSizeType;

public abstract class Beverage {

    private String description;

    private BeverageSizeType size;

    public Beverage(String description, BeverageSizeType size) {
        this.description = description;
        this.size = size;
    }

    public abstract Long getCost();

    public String getDescription() {
        return description;
    }

    public BeverageSizeType getSize() {
        return size;
    }
}
