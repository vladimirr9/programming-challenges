package org.example;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {

    private final Solution solution = new Solution();

    @Test
    void testSinglePair() {
        assertTrue(solution.isValid("()"));
    }

    @Test
    void testMultipleDifferentTypes() {
        assertTrue(solution.isValid("()[]{}"));
    }

    @Test
    void testIncorrectSinglePair() {
        assertFalse(solution.isValid("(]"));
    }

    @Test
    void testNestedValid() {
        assertTrue(solution.isValid("([])"));
    }

    @Test
    void testWrongOrder() {
        assertFalse(solution.isValid("([)]"));
    }

    // Extra useful edge cases
    @Test
    void testEmptyString() {
        assertTrue(solution.isValid(""));
    }

    @Test
    void testOnlyOpenBrackets() {
        assertFalse(solution.isValid("((("));
    }

    @Test
    void testOnlyCloseBrackets() {
        assertFalse(solution.isValid("]]]"));
    }

    @Test
    void testComplexValid() {
        assertTrue(solution.isValid("{[()()]}"));
    }

}