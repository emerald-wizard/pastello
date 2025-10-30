package config

import (
	"fmt"
	"os"

	"gopkg.in/yaml.v2"
)

type AppConfig struct {
	LogLevel string `yaml:"logLevel"`
	DB       struct {
		Host     string `yaml:"host"`
		Port     int    `yaml:"port"`
		User     string `yaml:"user"`
		Password string `yaml:"password"`
		Name     string `yaml:"name"`
	} `yaml:"db"`
}

func LoadConfig(path string) *AppConfig {
	data, err := os.ReadFile(path)
	if err != nil {
		fmt.Printf("Failed to read config file: %v\n", err)
		return &AppConfig{LogLevel: "info"} // fallback
	}
	var cfg AppConfig
	if err := yaml.Unmarshal(data, &cfg); err != nil {
		fmt.Printf("Failed to unmarshal config: %v\n", err)
		return &AppConfig{LogLevel: "info"}
	}
	return &cfg
}
