package com.example.guide_service.controller;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.bind.annotation.RequestMapping;

import java.time.Instant;
import java.util.Map;

@RestController
@RequestMapping("/health")
public class HealthController {

    @GetMapping
    public Map<String, String> getHealth() {
        return Map.of(
                "status", "ok",
                "service", "guide-service",
                "timestamp", Instant.now().toString()
        );
    }
}