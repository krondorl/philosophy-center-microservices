/*!
 * Philosophy Center Microservices
 *
 * Copyright (c) 2026- Adam Burucs
 *
 * MIT Licensed
 */

import { Controller, Get, Param, Query } from '@nestjs/common';
import { Quote } from './quote.types';
import { QuotesService } from './quotes.service';

@Controller('quotes')
export class QuotesController {
  constructor(private readonly quotesService: QuotesService) {}

  /**
   * GET /quotes
   * GET /quotes?philosopherId=marcus-aurelius
   */
  @Get()
  findAll(@Query('philosopherId') philosopherId?: string): Promise<Quote[]> {
    return this.quotesService.findAll(philosopherId);
  }

  /**
   * GET /quotes/random
   *
   * This route must be declared before GET /quotes/:id.
   */
  @Get('random')
  findRandom(): Promise<Quote> {
    return this.quotesService.findRandom();
  }

  /**
   * GET /quotes/{id}
   */
  @Get(':id')
  findById(@Param('id') id: string): Promise<Quote> {
    return this.quotesService.findById(id);
  }
}
