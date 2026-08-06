import { Injectable, NotFoundException } from '@nestjs/common';
import { Quote, QuoteRow } from './quote.types';
import { QuotesRepository } from './quotes.repository';

@Injectable()
export class QuotesService {
  constructor(private readonly quotesRepository: QuotesRepository) {}

  /**
   * GET /quotes
   * GET /quotes?philosopherId={id}
   */
  async findAll(philosopherId?: string): Promise<Quote[]> {
    const normalizedPhilosopherId = philosopherId?.trim();

    const rows = normalizedPhilosopherId
      ? await this.quotesRepository.findByPhilosopherId(normalizedPhilosopherId)
      : await this.quotesRepository.findAll();

    return rows.map((row) => this.mapToQuote(row));
  }

  /**
   * GET /quotes/{id}
   */
  async findById(id: string): Promise<Quote> {
    const row = await this.quotesRepository.findById(id);

    if (!row) {
      throw new NotFoundException(`Quote with id "${id}" was not found`);
    }

    return this.mapToQuote(row);
  }

  /**
   * GET /quotes/random
   */
  async findRandom(): Promise<Quote> {
    const row = await this.quotesRepository.findRandom();

    if (!row) {
      throw new NotFoundException('No quotes are available');
    }

    return this.mapToQuote(row);
  }

  private mapToQuote(row: QuoteRow): Quote {
    return {
      id: row.id,
      philosopherId: row.philosopherId,
      text: row.text,
    };
  }
}
