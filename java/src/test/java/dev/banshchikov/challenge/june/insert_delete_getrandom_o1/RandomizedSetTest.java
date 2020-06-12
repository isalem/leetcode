package dev.banshchikov.challenge.june.insert_delete_getrandom_o1;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class RandomizedSetTest {

    @Test
    void test1() {
        RandomizedSet randomizedSet = new RandomizedSet();
        randomizedSet.insert(1);
        randomizedSet.remove(2);
        randomizedSet.insert(2);
        randomizedSet.getRandom();
        randomizedSet.remove(1);
        assertEquals(2, randomizedSet.getRandom());
    }

    @Test
    void test2() {
        RandomizedSet randomizedSet = new RandomizedSet();
        randomizedSet.remove(0);
        randomizedSet.remove(0);
        randomizedSet.insert(0);
        randomizedSet.getRandom();
        randomizedSet.remove(0);
        randomizedSet.insert(0);
    }
}
