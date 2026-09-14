# M16 B1 — portable raw-shard replay

PROBATION / proof-present data / OPEN / UNMERGED. Rung: 0088-UI-PROTOTYPE-0.
Authority: Board 5654699942 and disposition 5671634736.
Retires with the rung; this is evidence, not a permanent instrument or authority.

Extract the single Python block to a temporary replay_shards.py and run `python replay_shards.py /path/to/checkout`. Python 3.13.2 standard library only. No local archive, original C: path, Studio, GPU or network is required. The verifier reconstructs every complete original capture, checks semantic SHA-256, replays original analyzers/protocols, checks all exact/ordinal residuals and sample coverage, and recomputes the primary/replication summaries. Markdown shard size/hash checks normalize CRLF to LF, permitting ordinary Git Windows checkouts; original artifact byte hashes remain unchanged. Optional `--selftest` checks detection of a changed tail, dropped sample and altered negative residual. Outputs are local derived evidence, not a new instrument or physical run.

<!-- m16-block:replay -->
```python
"""Verify committed M16 evidence without its original machine paths or ZIP."""
from collections import Counter
import copy
import csv
import hashlib
import io
import json
from pathlib import Path
import re
import sys

sys.dont_write_bytecode = True
PREFIX = 'rehearsal_perf_m16_ui_comparison_'

def sha(data):
    return hashlib.sha256(data).hexdigest()

def canon(value):
    return sha(json.dumps(value, sort_keys=True, separators=(',', ':'), ensure_ascii=True, allow_nan=False).encode())

def blocks(path):
    text = path.read_text(encoding='utf-8')
    return {m.group(1): (m.group(2), m.group(3)) for m in re.finditer(
        r'<!-- m16-block:([a-z_]+) -->\n```(csv|jsonl|python)\n(.*?)^```', text, re.S | re.M)}

def json_rows(section):
    assert section[0] == 'jsonl'
    return [json.loads(line) for line in section[1].splitlines() if line]

def csv_rows(section):
    assert section[0] == 'csv'
    return list(csv.DictReader(io.StringIO(section[1])))

def verify(repo, selftest=False):
    docs = repo / 'docs/tests'
    index_path = docs / (PREFIX + 'attempt_index_results.md')
    index = blocks(index_path)
    control = json_rows(index['control'])[0]
    assert control['schema'] == 'm16-b1-markdown-v1' and control['authority'] == '5671634736'
    parts = {}
    for file in control['data_shards']:
        path = docs / file['name']
        document_bytes = path.read_text(encoding='utf-8').encode('utf-8')
        assert len(document_bytes) == file['bytes'] and sha(document_bytes) == file['sha256'], file['name']
        for name, section in blocks(path).items():
            assert name not in parts
            parts[name] = section
    assert set(control['packet_index_metadata']) == {'snapshot_through','exclusions'}
    packet_index = {'snapshot_through':control['packet_index_metadata']['snapshot_through'],
                    'files':json_rows(index['archive_index']), 'exclusions':control['packet_index_metadata']['exclusions']}
    packet_bytes = (json.dumps(packet_index, indent=2) + '\n').encode()
    assert sha(packet_bytes) == control['raw_archive_index_sha256']
    captures = {c['capture_id']: c for c in json_rows(index['captures'])}
    runs = {r['run']: r for r in json_rows(index['runs'])}
    assert list(captures) == list(range(1, control['capture_count'] + 1)) and len(runs) == control['run_count']
    raw = {key: dict(c['base'], samples=[None] * c['sample_count']) for key,c in captures.items()}
    counts = Counter()

    def add(cap_id, sample_id, sample):
        target = raw[cap_id]['samples']
        assert 0 <= sample_id < len(target) and target[sample_id] is None, 'Duplicate or out-of-range sample'
        target[sample_id] = sample
        counts[sample['kind']] += 1

    scopes = {r['scope_id']: r['scope'] for r in json_rows(parts['scopes'])}
    for row in csv_rows(parts['frames']):
        add(int(row['capture_id']), int(row['sample_id']),
            dict(kind='Frame', start_ns=int(row['start_ns']), end_ns=int(row['end_ns']),
                 delta_ns=int(row['delta_ns']), time_strategy=row['time_strategy']))
    for row in csv_rows(parts['cpu']):
        add(int(row['capture_id']), int(row['sample_id']),
            dict(kind='Cpu', client=row['client'], scope=scopes[int(row['scope_id'])],
                 start_ns=int(row['start_ns']), end_ns=int(row['end_ns'])))
    for row in json_rows(parts['events']):
        add(row['capture_id'], row['sample_id'], row['sample'])
    assert dict(counts) == control['source_counts']
    for cap_id,capture in raw.items():
        assert all(sample is not None for sample in capture['samples']), 'Missing sample'
        assert canon(capture) == captures[cap_id]['raw_canonical_sha256'], 'Raw numeric/sample identity changed'
    versions, analyzers = {}, {}
    for version in json_rows(parts['versions']):
        assert sha(version['utf8_text'].encode('utf-8')) == version['sha256']
        versions[version['sha256']] = version
        if version['kind'] == 'analyzer':
            namespace = {'__name__':'original_b1_analyzer', '__file__':'original_analyzer.py'}
            exec(compile(version['utf8_text'].lstrip('\ufeff'), 'original_analyzer.py', 'exec'), namespace)
            analyzers[version['sha256']] = namespace
    memory = {m['capture_id']: m for m in json_rows(parts['memory'])}
    assert set(memory) == set(captures)
    reports, expected_cpu, expected_fresh = {}, [], []
    for cap_id,c in captures.items():
        run = runs[c['run']]
        assert cap_id in run['capture_ids']
        assert c['analyzer_sha256'] == run['analyzer_sha256'] and c['protocol_sha256'] == run['protocol_sha256']
        protocol = json.loads(versions[c['protocol_sha256']]['utf8_text'].lstrip('\ufeff'))
        report = analyzers[c['analyzer_sha256']]['analyze'](raw[cap_id], protocol)
        report.update(raw_sha256=c['raw_sha256'], analyzer_sha256=c['analyzer_sha256'], protocol_sha256=c['protocol_sha256'])
        assert canon(report) == c['report_canonical_sha256'], 'Original report differs'
        assert report['valid'] == c['valid'] and report['failures'] == c['failures']
        reports[cap_id] = report
        mem = memory[cap_id]
        assert canon(mem['record']) == mem['canonical_sha256']
        if mem['record'] is not None:
            assert mem['record']['process_id'] == run['process']['process_id']
            assert mem['record']['process_started_utc'] == run['process']['started_utc']
        for p in report['same_frame_same_publication_projection_pairs']:
            expected_cpu.append([cap_id, p['frame_index'], p['frame_start_ns'], p['frame_end_ns'], *p['publication_key'],
                                 p['egui_start_ns'], p['egui_end_ns'], p['native_start_ns'], p['native_end_ns'], p['native_minus_egui_ns']])
        for p in report['same_publication_freshness_pairs']:
            expected_fresh.append([cap_id, *[p['stamp'][k] for k in ('scene','resident_epoch','generation','published_ns')], p['native_minus_egui_ns']])
    actual_cpu = [[int(v) for v in row.values()] for row in csv_rows(parts['cpu_pairs'])]
    actual_fresh = [[int(v) for v in row.values()] for row in csv_rows(parts['freshness_pairs'])]
    assert actual_cpu == expected_cpu and actual_fresh == expected_fresh, 'Exact residual/identity mismatch'
    completed_paused = [name for name,run in runs.items() if len(run['capture_ids']) == 6
                        and all(reports[i]['valid'] and reports[i]['condition'] == 'paused' for i in run['capture_ids'])]
    assert control['primary_runs']['paused'] == min(completed_paused)
    for cap_id,c in captures.items():
        expected_role = ('primary' if c['run'] in control['primary_runs'].values()
                         else 'independent-replication' if c['run'] == control['replication_run']
                         else 'unpaired-valid' if reports[cap_id]['valid'] else 'rejected')
        assert c['role'] == expected_role
    stats = next(iter(analyzers.values()))['stats']
    pairs, expected_ordinal = [], []
    for role in ('primary', 'independent-replication'):
        conditions = list(control['primary_runs']) if role == 'primary' else ['paused']
        for condition in conditions:
            for repetition in (1,2,3):
                identities = {reports[i]['client']: i for i,c in captures.items() if c['role'] == role
                              and reports[i]['condition'] == condition and reports[i]['repetition'] == repetition}
                assert set(identities) == {'Egui','Native'}
                e,n = identities['Egui'],identities['Native']
                assert reports[e]['valid'] and reports[n]['valid']
                ef = [s['delta_ns'] for s in raw[e]['samples'] if s['kind'] == 'Frame']
                nf = [s['delta_ns'] for s in raw[n]['samples'] if s['kind'] == 'Frame']
                for i in range(max(len(ef),len(nf))):
                    ev,nv = ef[i] if i<len(ef) else None,nf[i] if i<len(nf) else None
                    expected_ordinal.append([role,condition,str(repetition),str(e),str(n),str(i),
                                             '' if ev is None else str(ev),'' if nv is None else str(nv),
                                             '' if ev is None or nv is None else str(nv-ev)])
                em,nm = memory[e]['record'],memory[n]['record']
                assert em is not None and nm is not None
                pairs.append({'role':role,'condition':condition,'repetition':repetition,'egui_capture_id':e,'native_capture_id':n,
                              'frame_native_minus_egui_ms':{k:reports[n]['frame_ms'][k]-reports[e]['frame_ms'][k]
                                                          for k in ('min','max','mean','p50','p95','p99')},
                              'memory_native_minus_egui_bytes':{k:nm[k]-em[k] for k in ('private_bytes','working_set_bytes')},
                              'ordinal_paired':min(len(ef),len(nf)), 'unmatched_intervals':abs(len(ef)-len(nf))})
    assert [list(r.values()) for r in csv_rows(parts['ordinal_pairs'])] == expected_ordinal, 'Ordinal or unmatched tail mismatch'
    falsifiers = []
    if selftest:
        first = next(i for i,c in captures.items() if c['role']=='primary')
        mutant = copy.deepcopy(raw[first])
        next(s for s in reversed(mutant['samples']) if s['kind']=='Frame')['delta_ns'] += 1
        assert canon(mutant) != captures[first]['raw_canonical_sha256']
        falsifiers.append('changed final frame interval detected by original canonical hash')
        mutant = copy.deepcopy(raw[first]); mutant['samples'].pop()
        assert canon(mutant) != captures[first]['raw_canonical_sha256']
        falsifiers.append('dropped final sample detected by original count/hash')
        mutant_pairs = copy.deepcopy(actual_cpu)
        next(row for row in mutant_pairs if row[-1]<0)[-1] += 1
        assert mutant_pairs != expected_cpu
        falsifiers.append('altered negative direct CPU residual detected by exact recomputation')
    capture_summaries = []
    for i,c in captures.items():
        r = reports[i]
        runtime = [{'at_ns':raw[i]['started_ns'],'facts':raw[i]['initial_facts']}]
        runtime += [s for s in raw[i]['samples'] if s['kind']=='Runtime']
        capture_summaries.append({'capture_id':i,'run':c['run'],'role':c['role'],'client':r['client'],'condition':r['condition'],
                                 'repetition':r['repetition'],'valid':r['valid'],'failures':r['failures'],
                                 'duration_seconds':r['duration_seconds'],'initial_facts':r['initial_facts'],
                                 'frame_ms':r['frame_ms'],'cpu_scopes_ns':r['cpu_scopes_ns'],'freshness_ns':r['freshness_ns'],
                                 'pre_capture_publication_ages':r['pre_capture_publication_ages'],
                                 'direct_cpu_pair_count':len(r['same_frame_same_publication_projection_pairs']),
                                 'direct_cpu_residual_ns':stats([p['native_minus_egui_ns'] for p in r['same_frame_same_publication_projection_pairs']]),
                                 'freshness_pair_count':len(r['same_publication_freshness_pairs']),
                                 'freshness_residual_ns':stats([p['native_minus_egui_ns'] for p in r['same_publication_freshness_pairs']]),
                                 'unpaired_projection_calls':r['unpaired_projection_calls'],'retained_rejections':r['retained_rejections'],
                                 'memory':memory[i]['record'],'runtime':runtime,'stopped_ns':raw[i]['stopped_ns']})
    return {'verdict':'PASS','captures':len(captures),'runs':len(runs),'valid':sum(r['valid'] for r in reports.values()),
            'rejected':sum(not r['valid'] for r in reports.values()),'roles':dict(Counter(c['role'] for c in captures.values())),
            'source_counts':dict(counts),'exact_cpu_pairs':len(expected_cpu),'exact_freshness_pairs':len(expected_fresh),
            'ordinal_rows_including_unmatched':len(expected_ordinal),'falsification_checks':falsifiers,
            'captures_recomputed':capture_summaries,'pairs':pairs}

if __name__ == '__main__':
    repo = Path(next((arg for arg in sys.argv[1:] if arg != '--selftest'), '.')).resolve()
    print(json.dumps(verify(repo, '--selftest' in sys.argv), indent=2, allow_nan=False))
```
