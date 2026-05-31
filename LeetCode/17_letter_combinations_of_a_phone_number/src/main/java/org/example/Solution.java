package org.example;


import java.util.ArrayList;
import java.util.List;

public class Solution {
    static void main() {

    }


    public List<String> letterCombinations(String digits) {
        List<String> letterCombinations = getAllLetters(digits.charAt(0));
        for (char digit : digits.substring(1).toCharArray()) {
            List<String> letters = getAllLetters(digit);
            List<String> newLetterCombinations = new ArrayList<>();
            for (String existingLetterCombination : letterCombinations) {
                for (String newLetter : letters) {
                    newLetterCombinations.add(existingLetterCombination + newLetter);
                }
            }
            letterCombinations = newLetterCombinations;
        }
        return letterCombinations;
    }

    private static List<String> getAllLetters(char c) {
        return switch (c) {
            case '2' -> List.of("a", "b", "c");
            case '3' -> List.of("d", "e", "f");
            case '4' -> List.of("g", "h", "i");
            case '5' -> List.of("j", "k", "l");
            case '6' -> List.of("m", "n", "o");
            case '7' -> List.of("p", "q", "r", "s");
            case '8' -> List.of("t", "u", "v");
            case '9' -> List.of("w", "x", "y", "z");
            default -> List.of();
        };
    }
}
