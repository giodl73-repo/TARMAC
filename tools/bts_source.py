"""Verify/rederive the frozen BTS HTML using only Python's standard library."""
import hashlib
from html.parser import HTMLParser
import json
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
FIELDS = ['on_time', 'on_time_percent_hundredths', 'delayed',
          'delayed_percent_hundredths', 'cancelled',
          'cancelled_percent_hundredths', 'diverted', 'flight_operations']


class Table(HTMLParser):
    def __init__(self):
        super().__init__()
        self.rows, self.cells, self.text, self.notes = [], [], None, []
        self.select, self.selected, self.filters = None, False, {}

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if tag == 'select':
            self.select = attrs.get('name')
        if tag == 'option':
            self.selected = 'selected' in attrs
        if tag == 'tr':
            self.cells = []
        if tag in ('td', 'th'):
            self.text = []
        if tag == 'br' and self.text is not None:
            self.text.append(' ')

    def handle_data(self, data):
        if self.text is not None:
            self.text.append(data)
        if self.selected and data.strip():
            self.filters[self.select] = data.strip()
            self.selected = False

    def handle_endtag(self, tag):
        if tag == 'select':
            self.select, self.selected = None, False
        if tag in ('td', 'th') and self.text is not None:
            text = ' '.join(''.join(self.text).split())
            self.cells.append(text)
            if text.startswith('* For purposes of this report'):
                self.notes.append(text)
            self.text = None
        if tag == 'tr' and len(self.cells) == 9 and re.fullmatch(r'20\d\d', self.cells[0]):
            self.rows.append(self.cells.copy())


def derive():
    custody = json.loads((ROOT / 'data/bts/custody.json').read_text())
    raw = (ROOT / 'data/bts/transtats-arrivals.html').read_bytes()
    assert hashlib.sha256(raw).hexdigest() == custody['sha256'], 'Original HTML hash changed'
    assert len(raw) == custody['bytes']
    text = raw.decode('utf-8')
    assert 'January to August' in text and 'Reporting Operating Carrier Flight Delays at a Glance' in text
    parser = Table()
    parser.feed(text)
    assert parser.filters == {'Airport': 'All Airports', 'Carrier': 'All Carriers',
                              'CboTime': 'Year-To-Date', 'CboSelection': 'Arrivals'}
    assert len(parser.notes) == 1 and len(parser.rows) == 10
    rows = []
    for raw_row in parser.rows:
        values = {}
        for field, value in zip(FIELDS, raw_row[1:]):
            if field.endswith('_hundredths'):
                assert re.fullmatch(r'\d+\.\d{2}%', value)
                values[field] = int(value.replace('.', '').replace('%', ''))
            else:
                assert re.fullmatch(r'[\d,]+', value)
                values[field] = int(value.replace(',', ''))
        assert sum(values[k] for k in ['on_time', 'delayed', 'cancelled', 'diverted']) == values['flight_operations']
        rows.append(dict(year=int(raw_row[0]), published=raw_row[1:], **values))
    assert [r['year'] for r in rows] == list(range(2017, 2027))
    return dict(source_id='bts-transtats-arrivals-2026-10-24', url=custody['url'],
                window='January-August', scale='national', filters=parser.filters,
                captured_at=custody['captured_at'], raw_sha256=custody['sha256'],
                notes=parser.notes, rows=rows)


def verify(write=False):
    value = derive()
    path = ROOT / 'fixtures/bts/summary.json'
    if write:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes((json.dumps(value, indent=2, ensure_ascii=False) + '\n').encode())
    else:
        assert json.loads(path.read_text(encoding='utf-8')) == value, 'Normalized source differs'
    print('BTS original hash, filters, 10 rows and count reconciliation verified')


if __name__ == '__main__':
    verify('--write' in sys.argv)
