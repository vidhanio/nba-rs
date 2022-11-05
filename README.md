# `nba-rs`

## Selector

```javascript
document
    .querySelector(".nba-stats-primary-split-block")
    .querySelectorAll(".DropDown_label__lttfI")
    .forEach(label => console.log(
        label.querySelector("p").innerText, 
        [...label.querySelectorAll("option")].map(option => 
            [option.selected ? "DEFAULT" : "", option.innerText, option.value]
        )
    ))
```
