/*!
 * Philosophy Center Microservices
 *
 * Copyright (c) 2026- Adam Burucs
 *
 * MIT Licensed
 */

package com.example.guide_service.repository;

import com.example.guide_service.model.Guide;
import org.springframework.jdbc.core.simple.JdbcClient;
import org.springframework.stereotype.Repository;

import java.util.List;
import java.util.Optional;
import java.util.UUID;

@Repository
public class GuideRepository {

    private final JdbcClient jdbcClient;

    public GuideRepository(JdbcClient jdbcClient) {
        this.jdbcClient = jdbcClient;
    }

    public List<Guide> findAll() {
        return jdbcClient.sql("""
                SELECT
                    g.id,
                    g.slug,
                    g.title,
                    g.school_id,
                    COALESCE(
                        array_agg(gp.philosopher_id)
                            FILTER (WHERE gp.philosopher_id IS NOT NULL),
                        ARRAY[]::text[]
                    ) AS philosopher_ids
                FROM guides g
                LEFT JOIN guide_philosophers gp
                    ON gp.guide_id = g.id
                GROUP BY g.id, g.slug, g.title, g.school_id
                ORDER BY g.title
                """)
                .query((rs, rowNum) -> new Guide(
                        rs.getObject("id", UUID.class),
                        rs.getString("slug"),
                        rs.getString("title"),
                        rs.getString("school_id"),
                        List.of(
                                (String[]) rs.getArray("philosopher_ids")
                                        .getArray()
                        )
                ))
                .list();
    }

    public Optional<Guide> findById(UUID id) {
        return jdbcClient.sql("""
                SELECT
                    g.id,
                    g.slug,
                    g.title,
                    g.school_id,
                    COALESCE(
                        array_agg(gp.philosopher_id)
                            FILTER (WHERE gp.philosopher_id IS NOT NULL),
                        ARRAY[]::text[]
                    ) AS philosopher_ids
                FROM guides g
                LEFT JOIN guide_philosophers gp
                    ON gp.guide_id = g.id
                WHERE g.id = :id
                GROUP BY g.id, g.slug, g.title, g.school_id
                """)
                .param("id", id)
                .query((rs, rowNum) -> new Guide(
                        rs.getObject("id", UUID.class),
                        rs.getString("slug"),
                        rs.getString("title"),
                        rs.getString("school_id"),
                        List.of(
                                (String[]) rs.getArray("philosopher_ids")
                                        .getArray()
                        )
                ))
                .optional();
    }

    public List<Guide> findBySchoolId(String schoolId) {
        return jdbcClient.sql("""
                SELECT
                    g.id,
                    g.slug,
                    g.title,
                    g.school_id,
                    COALESCE(
                        array_agg(gp.philosopher_id)
                            FILTER (WHERE gp.philosopher_id IS NOT NULL),
                        ARRAY[]::text[]
                    ) AS philosopher_ids
                FROM guides g
                LEFT JOIN guide_philosophers gp
                    ON gp.guide_id = g.id
                WHERE g.school_id = :schoolId
                GROUP BY g.id, g.slug, g.title, g.school_id
                ORDER BY g.title
                """)
                .param("schoolId", schoolId)
                .query((rs, rowNum) -> new Guide(
                        rs.getObject("id", UUID.class),
                        rs.getString("slug"),
                        rs.getString("title"),
                        rs.getString("school_id"),
                        List.of(
                                (String[]) rs.getArray("philosopher_ids")
                                        .getArray()
                        )
                ))
                .list();
    }
}