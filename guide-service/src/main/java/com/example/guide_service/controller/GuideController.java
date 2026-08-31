package com.example.guide_service.controller;

import com.example.guide_service.model.Guide;
import com.example.guide_service.repository.GuideRepository;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;
import java.util.UUID;

@RestController
@RequestMapping("/guides")
public class GuideController {

    private final GuideRepository guideRepository;

    public GuideController(GuideRepository guideRepository) {
        this.guideRepository = guideRepository;
    }

    @GetMapping
    public List<Guide> getGuides(
            @RequestParam(required = false) String schoolId
    ) {
        if (schoolId != null) {
            return guideRepository.findBySchoolId(schoolId);
        }

        return guideRepository.findAll();
    }

    @GetMapping("/{id}")
    public ResponseEntity<Guide> getGuide(@PathVariable UUID id) {
        return guideRepository.findById(id)
                .map(ResponseEntity::ok)
                .orElseGet(() -> ResponseEntity.notFound().build());
    }
}