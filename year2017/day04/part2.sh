awk '
    BEGIN{
        alphabet_len = split("abcdefghijklmnopqrstuvwxyz", alphabet, "");
        primes_len = split("2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101", primes, ",");
        if (alphabet_len != 26 || primes_len != 26) {
            print "Setup error";
            exit 1;
        }
        for (i in alphabet) {
            char_to_prime[alphabet[i]] = primes[i];
        }
    }

    function word_to_numeric(word) {
        split(word, chars, "");
        numeric = 1;
        for (ci in chars) {
            numeric *= char_to_prime[chars[ci]];
        }
        return numeric;
    }

    {
        delete seen;
        for (i=1; i<=NF; ++i) {
            n = word_to_numeric($i);
            if (seen[n]++) {
                next;
            }
        }
        sol += 1;
    }

    END{print sol}
' <input.txt
