/*!
 * Philosophy Center Microservices
 *
 * Copyright (c) 2026- Adam Burucs
 *
 * MIT Licensed
 */

import { Module } from '@nestjs/common';
import { DatabaseModule } from '../database/database.module';
import { QuotesController } from './quotes.controller';
import { QuotesService } from './quotes.service';
import { QuotesRepository } from './quotes.repository';

@Module({
  imports: [DatabaseModule],
  controllers: [QuotesController],
  providers: [QuotesService, QuotesRepository],
})
export class QuotesModule {}
