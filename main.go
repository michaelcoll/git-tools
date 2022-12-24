package main

import (
	"fmt"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"strings"

	"github.com/fatih/color"
)

func main() {
	args := os.Args[1:]
	if len(args) <= 0 {
		fmt.Println("Folder argument needed !")
		os.Exit(1)
	}

	currentDir := args[0]

	scan(currentDir)

}

// scan scans the given path
func scan(path string) {
	files, err := os.ReadDir(path)
	if err != nil {
		log.Fatal(err)
	}

	for _, file := range files {
		if file.Name() == ".git" {
			compactRepo(path)

		} else if file.IsDir() {
			scan(filepath.Join(path, file.Name()))
		}
	}
}

// getSizeOfDir returns the size of the given path
func getSizeOfDir(path string) float64 {
	cmd := exec.Command("du", "-s", path)
	stdout, err := cmd.Output()
	if err != nil {
		log.Fatal(err)
	}

	size, err := strconv.Atoi(strings.Split(string(stdout), "\t")[0])
	if err != nil {
		log.Fatal(err)
	}

	return float64(size)
}

// compactRepo compresses the git repo with the given path
func compactRepo(path string) {
	pathStr := path
	if !strings.HasPrefix(path, "/") && path != "." {
		pathStr = "./" + path
	}

	fmt.Print(pathStr)

	sizeBefore := getSizeOfDir(path)

	execGitGC(path)

	sizeAfter := getSizeOfDir(path)

	diff := sizeBefore - sizeAfter
	diffStr := fmt.Sprintf("(%.2fM)", diff/1024)
	ratio := fmt.Sprintf("%.2f%%", (diff/sizeBefore)*100)

	if diff > 0 {
		fmt.Printf(" — Saved %s ", color.GreenString(ratio))
		if _, err := color.New(color.FgHiBlack).Println(diffStr); err != nil {
			log.Fatal(err)
		}
	} else {
		if _, err := color.New(color.FgHiBlack).Println(" — No change"); err != nil {
			log.Fatal(err)
		}
	}
}

// execGitGC execute the git gc command on the given path
func execGitGC(path string) {

	cmd := exec.Command("git", "gc", "--aggressive")
	cmd.Dir = path

	if _, err := cmd.Output(); err != nil {
		log.Fatal(err)
	}

}
