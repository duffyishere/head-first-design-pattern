package org.example.size;

public enum BeverageSizeType {

    TALL(0L),
    GRANDE(1500L),
    VENTI(3000L);

    private Long cost;

    BeverageSizeType(Long cost) {
        this.cost = cost;
    }
}
