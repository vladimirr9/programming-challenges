package org.example;

import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {

    private final Solution solution = new Solution();

    @Test
     void testExample23() {
        List<String> result = solution.letterCombinations("23");

        assertEquals(9, result.size());
        assertTrue(result.containsAll(List.of(
                "ad", "ae", "af",
                "bd", "be", "bf",
                "cd", "ce", "cf"
        )));
    }

    @Test
    void testSingleDigit2() {
        List<String> result = solution.letterCombinations("2");

        assertEquals(List.of("a", "b", "c"), result);
    }

    @Test
    void testSingleDigit7() {
        List<String> result = solution.letterCombinations("7");

        assertEquals(4, result.size());
        assertTrue(result.containsAll(List.of("p", "q", "r", "s")));
    }

}