package ports

type RNG interface{ Intn(n int) int }
