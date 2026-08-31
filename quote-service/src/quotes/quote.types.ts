/*!
 * Philosophy Center Microservices
 *
 * Copyright (c) 2026- Adam Burucs
 *
 * MIT Licensed
 */

import { QueryResultRow } from 'pg';

export interface QuoteRow extends QueryResultRow {
  id: string;
  philosopherId: string;
  quoteText: string;
}

export interface Quote {
  id: string;
  philosopherId: string;
  quoteText: string;
}
