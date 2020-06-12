package dev.banshchikov.challenge.june.insert_delete_getrandom_o1;

import java.util.*;
import java.util.concurrent.ThreadLocalRandom;

class RandomizedSet {

    private final List<Integer> storage = new ArrayList<>();
    private final Map<Integer, Integer> index = new HashMap<>();

    /** Initialize your data structure here. */
    public RandomizedSet() {

    }

    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    public boolean insert(int val) {
        if (index.containsKey(val)) {
            return false;
        }

        storage.add(val);
        int pos = storage.indexOf(val);

        index.put(val, pos);

        return true;
    }

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    public boolean remove(int val) {
        if (!index.containsKey(val)) {
            return false;
        }

        int pos = index.get(val);
        int last = storage.remove(storage.size() - 1);

        if (last != val) {
            storage.set(pos, last);
            index.put(last, pos);
        }

        index.remove(val);

        return true;
    }

    /** Get a random element from the set. */
    public int getRandom() {
        int pos = ThreadLocalRandom.current().nextInt(0, storage.size());
        return storage.get(pos);
    }
}
