/*!
 * Philosophy Center Microservices
 *
 * Copyright (c) 2026- Adam Burucs
 *
 * MIT Licensed
 */

import { Injectable } from '@nestjs/common';
import { DatabaseService } from '../database/database.service';
import { QuoteRow } from './quote.types';

@Injectable()
export class QuotesRepository {
  constructor(private readonly databaseService: DatabaseService) {}

  async findAll(): Promise<QuoteRow[]> {
    const result = await this.databaseService.query<QuoteRow>(
      `
        SELECT
          id,
          philosopher_id AS "philosopherId",
          quote_text AS "quoteText"
        FROM quotes
        LIMIT 20
      `,
    );

    return result.rows;
  }

  async findByPhilosopherId(philosopherId: string): Promise<QuoteRow[]> {
    const result = await this.databaseService.query<QuoteRow>(
      `
        SELECT
          id,
          philosopher_id AS "philosopherId",
          quote_text AS "quoteText"
        FROM quotes
        WHERE philosopher_id = $1
        LIMIT 10
      `,
      [philosopherId],
    );

    return result.rows;
  }

  /**
   * Returns one random quote.
   */
  async findRandom(): Promise<QuoteRow | null> {
    const result = await this.databaseService.query<QuoteRow>(
      `
        SELECT
          id,
          philosopher_id AS "philosopherId",
          quote_text AS "quoteText"
        FROM quotes
        ORDER BY RANDOM()
        LIMIT 1
      `,
    );

    return result.rows[0] ?? null;
  }

  /**
   * Returns one quote by its ID.
   */
  async findById(id: string): Promise<QuoteRow | null> {
    const result = await this.databaseService.query<QuoteRow>(
      `
        SELECT
          id,
          philosopher_id AS "philosopherId",
          quote_text AS "quoteText"
        FROM quotes
        WHERE id = $1
        LIMIT 1
      `,
      [id],
    );

    return result.rows[0] ?? null;
  }
}
