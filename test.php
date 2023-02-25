<?php
// Score: 1
echo pronounceability('namelet') . "\n";

// Score: 0.71428571428571
echo pronounceability('nameoic') . "\n";

echo pronounceability('google') . "\n";

echo pronounceability('vresque') . "\n";

function pronounceability($word) {
    static $vowels = array
        (
        'a',
        'e',
        'i',
        'o',
        'u',
        'y'
        );

    static $composites = array
        (
        'mm',
        'll',
        'th',
        'ing'
        );

    if (!is_string($word)) return false;

    // Remove non letters and put in lowercase
    $word = preg_replace('/[^a-z]/i', '', $word);
    $word = strtolower($word);

    // Special case
    if ($word == 'a') return 1;

    $len = strlen($word);

    // Let's not parse an empty string
    if ($len == 0) return 0;

    $score = 0;
    $pos = 0;

    while ($pos < $len) {
        // Check if is allowed composites
        foreach ($composites as $comp) {
            $complen = strlen($comp);

            if (($pos + $complen) < $len) {
                $check = substr($word, $pos, $complen);

                if ($check == $comp) {
                    $score += $complen;
                    $pos += $complen;
                    continue 2;
                }
            }
        }

        // Is it a vowel? If so, check if previous wasn't a vowel too.
        if (in_array($word[$pos], $vowels)) {
            if (($pos - 1) >= 0 && !in_array($word[$pos - 1], $vowels)) {
                $score += 1;
                $pos += 1;
                continue;
            }
        } else { // Not a vowel, check if next one is, or if is end of word
            if (($pos + 1) < $len && in_array($word[$pos + 1], $vowels)) {
                $score += 2;
                $pos += 2;
                continue;
            } elseif (($pos + 1) == $len) {
                $score += 1;
                break;
            }
        }

        $pos += 1;
    }

    return $score / $len;
}