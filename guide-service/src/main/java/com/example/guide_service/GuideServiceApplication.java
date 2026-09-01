/*!
 * Philosophy Center Microservices
 *
 * Copyright (c) 2026- Adam Burucs
 *
 * MIT Licensed
 */

package com.example.guide_service;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.boot.CommandLineRunner;
import org.springframework.context.annotation.Bean;

@SpringBootApplication
public class GuideServiceApplication {

    @Value("${server.address:localhost}")
    private String serverAddress;

    @Value("${server.port:8080}")
    private int serverPort;

    public static void main(String[] args) {
        SpringApplication.run(GuideServiceApplication.class, args);
    }

    @Bean
    CommandLineRunner printStartupMessage() {
        return args -> {
            System.out.println();
            System.out.println("Guide Service started");
            System.out.println("Server started at http://" + serverAddress + ":" + serverPort);
        };
    }
}
