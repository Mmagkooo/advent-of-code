awk '
    {
        split($0, chars, "");
        len=length($0)
    }
    {
        for (i=1; i<=len; ++i) {
            if (chars[i] == chars[(i + len/2 - 1) % len + 1]) {
                sol+=chars[i]
            }
        }
    }
    END{ print sol }
' <input.txt
