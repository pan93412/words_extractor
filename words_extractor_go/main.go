package main

import (
	"fmt"
	"io/ioutil"
	"strings"
	"time"

	"github.com/jfcg/sorty"
	"github.com/thoas/go-funk"
)

type resultsArray struct {
	Results []string
}

var (
	res resultsArray
)

func timeTrack(start time.Time) {
	elapsed := time.Since(start)
	fmt.Println("Total timing: ", elapsed)
}

func main() {
	t1 := time.Now()
	defer timeTrack(t1)
	folder := "./words"
	prepareFolder(folder, "*.txt")

	for _, path := range getYamlFilepaths("../data/pl/") {
		meta := getMeta(path)
		filename := "słowa - " + meta.Label + ".txt"
		fmt.Println("Parsing...", filename)

		res.extractWords(getRows(path).toString())
		res.Results = funk.UniqString(res.Results)
		sorty.SortS(res.Results)
		data := strings.Join(res.Results, "\n")

		for err := ioutil.WriteFile(folder+"/"+filename, []byte(data), 0644); err != nil; {
			panic(err)
		}
	}
}
