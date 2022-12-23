#!/bin/zsh

echo "Timing $1..."

durations=$($1 | rg -o 'runner: .+')
regex="runner: ([0-9.]+)([msµn])s?"
nanos=0
final_durations=()
day=1
part=1

echo "\n--Durations--"
while IFS= read -r duration
do
    if [[ $duration =~ $regex ]]
    then
        value="$match[1]"
        unit="$match[2]"

        case $unit in
            n)
                converted=$value.
                ;;
            µ)
                converted=$((value * 1000))
                ;;
            m)
                converted=$((value * 1000 * 1000))
                ;;
            s)
                converted=$((value * 1000 * 1000 * 1000))
                ;;
        esac

        nanos=$((nanos + converted))
        display=$(printf "Day %02d Part $part    Time: %.3f ms" $day $((converted / 1000000)))
        printf " $display\n"
        final_durations+=("$display\n")

        ((part+=1))
        if [[ $part -gt 2 ]]
        then
            part=1
            ((day+=1))
        fi

    else
        echo "Invalid duration '$duration'" >&2
        exit 1;
    fi
done <<< "$durations"

echo "\n--10 slowest--"
echo "$(echo $final_durations | sort -k6 -r -n | head -n 10)"

millis=$((nanos / 1000000))
echo "\n--Result--\n Total runner duration: $millis ms\n"
