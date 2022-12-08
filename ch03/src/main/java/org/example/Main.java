package org.example;

import org.example.beverage.Beverage;
import org.example.beverage.DarkCoffee;
import org.example.condimentDecorator.Mocha;

public class Main {
    public static void main(String[] args) {
        Beverage darkCoffee = new DarkCoffee("아메리카노입니다.");
        System.out.println("설명: "+darkCoffee.getDescription()+", 가격: "+darkCoffee.getCost());

        darkCoffee = new Mocha(darkCoffee);
        System.out.println("설명: "+darkCoffee.getDescription()+", 가격: "+darkCoffee.getCost());
    }
}