awk -v RS='(.)' '
    {c=RT}
    NR==1{first=c}
    NR>1 && c==prev{sol+=c}
    {prev=c}
    END{if (c==first) sol+=c; print sol}
' input.txt
