package org.example;


import java.util.ArrayDeque;
import java.util.Deque;

public class Solution {
    static void main() {
    }

    public boolean isValid(String s) {
        Deque<Character> deque = new ArrayDeque<>();
        for (char c : s.toCharArray()) {
            if ("({[".contains(String.valueOf(c))) {
                deque.addFirst(c);
            } else {
                if (deque.isEmpty()) {
                    return false;
                }
                char popped = deque.pop();
                if (!isMatchingChar(popped, c)) {
                    return false;
                }
            }
        }
        return deque.isEmpty();
    }

    private static boolean isMatchingChar(char openParenthesis, char closedParenthesis) {
        if (openParenthesis == '(') {
            return closedParenthesis == ')';
        } else if (openParenthesis == '{') {
            return closedParenthesis == '}';
        } else {
            return closedParenthesis == ']';
        }
    }
}

