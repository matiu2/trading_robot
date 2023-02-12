## Tools

These are the tools we'll be using.

### Renko ATR(14)
 
### Higher high, lower low

```
//
// @author LonesomeTheBlue
//
//@version=3
study("Higher High Lower Low Strategy", overlay =true)
lb = input(2, title="Left Bars", minval = 1)
rb = input(2, title="Right Bars", minval = 1)

showsupres = input(true, title="Show Support/Resistance")
changebarcol = input(true, title="Change Bar Color")
mb = lb + rb + 1

ph = iff(not na(high[mb]), iff(highestbars(high, mb) == -lb, high[lb], na), na) // Pivot High
pl = iff(not na(low[mb]), iff(lowestbars(low, mb) == -lb, low[lb], na), na) // Pivot Low

hl = na
hl := iff(ph, 1, iff(pl, -1, na)) // Trend direction
zz = na
zz := iff(ph, ph, iff(pl, pl, na)) // similar to zigzag but may have multiple highs/lows
zz :=iff(pl and hl == -1 and valuewhen(hl, hl, 1) == -1 and pl > valuewhen(zz, zz, 1), na, zz)
zz :=iff(ph and hl == 1  and valuewhen(hl, hl, 1) == 1  and ph < valuewhen(zz, zz, 1), na, zz)

hl := iff(hl==-1 and valuewhen(hl, hl, 1)==1 and zz > valuewhen(zz, zz, 1), na, hl)
hl := iff(hl==1 and valuewhen(hl, hl, 1)==-1 and zz < valuewhen(zz, zz, 1), na, hl)
zz := iff(na(hl), na, zz)

findprevious()=>  // finds previous three points (b, c, d, e)
    ehl = iff(hl==1, -1, 1)
    loc1 = 0.0, loc2 = 0.0, loc3 = 0.0, loc4 = 0.0
    xx = 0
    for x=1 to 1000
        if hl[x]==ehl and not na(zz[x])
            loc1 := zz[x]
            xx := x + 1
            break
    ehl := hl
    for x=xx to 1000
        if hl[x]==ehl and not na(zz[x])
            loc2 := zz[x]
            xx := x + 1
            break
    ehl := iff(hl==1, -1, 1)
    for x=xx to 1000
        if hl[x]==ehl and not na(zz[x])
            loc3 := zz[x]
            xx := x + 1
            break
    ehl := hl
    for x=xx to 1000
        if hl[x]==ehl and not na(zz[x])
            loc4 := zz[x]
            break
    [loc1, loc2, loc3, loc4]

a = na, b = na, c = na, d = na, e = na
if not na(hl)
    [loc1, loc2, loc3, loc4] = findprevious()
    a := zz 
    b := loc1
    c := loc2
    d := loc3
    e := loc4

_hh = zz and (a > b and a > c and c > b and c > d)
_ll = zz and (a < b and a < c and c < b and c < d)
_hl = zz and ((a >= c and (b > c and b > d and d > c and d > e)) or (a < b and a > c and b < d))
_lh = zz and ((a <= c and (b < c and b < d and d < c and d < e)) or (a > b and a < c and b > d))

plotshape(_hl, text="HL", title="Higher Low", style=shape.labelup, color=lime, textcolor=black, location=location.belowbar, transp=0, offset = -lb)
plotshape(_hh, text="HH", title="Higher High", style=shape.labeldown, color=lime, textcolor=black, location=location.abovebar, transp=0, offset = -lb)
plotshape(_ll, text="LL", title="Lower Low", style=shape.labelup, color=red, textcolor=white, location=location.belowbar, transp=0, offset = -lb)
plotshape(_lh, text="LH", title="Lower High", style=shape.labeldown, color=red, textcolor=white, location=location.abovebar, transp=0, offset = -lb)

res = na, sup = na
res := iff(_lh, zz, res[1])
sup := iff(_hl, zz, sup[1])

trend = na
trend := iff(close > res, 1, iff(close < sup, -1, nz(trend[1])))

res := iff((trend == 1 and _hh) or (trend == -1 and _lh), zz, res)
sup := iff((trend == 1 and _hl) or (trend == -1 and _ll), zz, sup)

plot(showsupres ? res : na, title="Resistance", color= na(res) ? na : red, linewidth=2, style=circles, offset = -lb)
plot(showsupres ? sup : na, title="Support", color= na(sup) ? na : blue, linewidth=2, style=circles, offset = -lb)

barcolor(color = iff(changebarcol, iff(trend == 1, blue, black), na))
```

### Trend strategy

```
//@version=4


// Trend line part
study(title="Trend Strategy", overlay = true)
Length = input(21, minval=1),
Multiplier = input(3, minval=1)
avgTR      = wma(atr(1), Length)
highestC   = highest(Length)
lowestC    = lowest(Length)
hiLimit = highestC[1]-(avgTR[1] * Multiplier)
loLimit = lowestC[1]+(avgTR[1] * Multiplier)

var trend_marker = 0.0
trend_marker := iff(close > hiLimit and close > loLimit, hiLimit, iff(close < loLimit and close < hiLimit, loLimit, nz(trend_marker[1], 0)))
var pos = 0.0
pos := iff(close > trend_marker, 1, iff(close < trend_marker, -1, nz(pos[1], 0)))

barcolor(pos == -1 ? color.red: pos == 1 ? color.yellow : color.blue )
plot(trend_marker, color= color.blue , title="Trend Trader Strategy")

// Add a label showing how high a bar is in pips
height = abs(open - close)
if barstate.islast
    height_label = "block pips: " + tostring(height / (syminfo.mintick * 10), '####') + " -- SL: " + tostring(height * 2.5 / (syminfo.mintick * 10), '####')
    label.new(bar_index, high, height_label)
```