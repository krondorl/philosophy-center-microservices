package com.example.guide_service.model;

import java.util.List;
import java.util.UUID;

public record Guide(
        UUID id,
        String slug,
        String title,
        String schoolId,
        List<String> philosopherIds
) {}