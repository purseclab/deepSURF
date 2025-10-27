import subprocess
import os
import sys
import json
import copy
import time
import multiprocessing
import toml
import struct


def recur_scan(folder:str, ans: list):
    if not os.path.exists(folder):
        return
    for f in os.listdir(folder):
        path = folder +'/'+f
        if os.path.isfile(path) and (path.endswith(".rs")):
            ans.append(path)
        if os.path.isdir(path):
            recur_scan(path, ans)


head="""
    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {{
            if let Ok(({})) = <({}) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){{

"""

tail = "             }\n});"


def get_bytes(ty: str, data: str):
    if ty in ['i8', 'u8', 'i16', 'u16', 'i32', 'u32', 'i64', 'u64', 'i128', 'u128']:
        data = data.replace("_", '').replace(ty, '')
        val = 0
        for i in [10, 2, 8, 16]:
            try:
                val = int(data, i)
                break
            except:
                pass
        len = int(int(ty[1:])/8)
        try:
            return val.to_bytes(len, sys.byteorder)
        except:
            return val.to_bytes(16, sys.byteorder)
    elif ty in ['isize', 'usize']:
        data = data.replace("_", '').replace('isize', '').replace('usize', '')
        val = 0
        for i in [10, 2, 8, 16]:
            try:
                val = int(data, i)
                break
            except:
                pass
        len = 8
        try:
            return val.to_bytes(len, sys.byteorder)
        except:
            return val.to_bytes(16, sys.byteorder)
    elif ty in ['f32','f64']:
        data = data.replace("_", '').replace('f32', '').replace('f64', '')
        val = float(data)
        if ty == 'f32':
            return struct.pack('<f', val)
        else:
            return struct.pack('<d', val)
    elif ty in ['&str', '& str']:
        val = data[1:-1]
        return val.encode('utf-8')
    elif ty == 'char':
        val = ord(data[1]) % 128
        return val.to_bytes(1, sys.byteorder)
    elif ty == 'bool':
        val = data == 'true'
        return val.to_bytes(1, sys.byteorder)
    elif ty.startswith('&[u8'):
        val = data[2:-1]
        return val.encode('utf-8')
    else:
        print('unknown ty', ty, data)
        return None


def execute_one_fd(fd):
    fin = subprocess.run('cargo ws list -l', shell=True, cwd=fd, capture_output=True)
    uid = 0
    total = 0
    succeed = 0
    skip = 0
    # config = toml.load(fd +'/Cargo.toml')
    # if 'dependencies' not in config:
    #     config['dependencies'] = {}
    # config['dependencies']['bolero']="0.8.0"
    # config['dependencies']['arbitrary']="1.3.2"
    # if 'profile' not in config:
    #     config['profile'] = {}
    # if 'fuzz' not in config:
    #     config['profile']['fuzz'] = {}
    # config['profile']['fuzz']['inherits'] = "dev"
    # config['profile']['fuzz']['opt-level'] = 1
    # config['profile']['fuzz']['incremental'] = False
    # config['profile']['fuzz']['codegen-units'] = 1
    # with open(fd +'/Cargo.toml', 'r+') as fp:
    #     fp.truncate(0)
    #     fp.seek(0)
    #     toml.dump(config, fp)
    #     fp.flush()


    with open(fd+'/fuzz_driver.log', 'w') as sys.stdout:
        for l in fin.stdout.decode('utf-8').splitlines():
            ls = l.split(' ')
            crate = ls[0].strip()
            path = ls[-1]
            files = []
            recur_scan(os.getcwd() +'/'+fd+'/'+path+'/src',files)
            # config = toml.load(fd+'/' + path +'/Cargo.toml')
            # if 'dependencies' not in config:
            #     config['dependencies'] = {}
            # config['dependencies']['bolero']="0.8.0"
            # config['dependencies']['arbitrary']="1.3.2"
            # with open(fd +'/'+path +'/Cargo.toml', 'r+') as fp:
            #     fp.truncate(0)
            #     fp.seek(0)
            #     toml.dump(config, fp)
            #     fp.flush()
            # #
            # fin = subprocess.run("cargo test --no-run", shell=True, cwd=fd+'/'+path, capture_output=True)
            # print(fd, crate, path, fin.returncode, files)
            # continue
            ans = {}
            data = ''

            ret = subprocess.run("cargo clean && FUZZ_TEST=1 cargorunner rudra", shell=True, cwd=fd+'/'+path, capture_output=True)
            if ret.returncode != 0 or not os.path.exists(fd+'/'+crate+'_fuzz_trans.json'):
                print('err1 toml', fd, crate, path)
                continue
            assert ret.returncode == 0
            with open(fd+'/'+crate+'_fuzz_trans.json', 'r') as fp:
                data = json.load(fp)
            if os.path.exists(fd+'/'+path+'/inputs'):
                subprocess.run("rm -rf {}".format(fd+'/'+path+'/inputs'), shell=True, capture_output=True)
            os.mkdir(fd+'/'+path+'/inputs')
            tys = {}
            for l in ret.stdout.decode('utf-8').splitlines():
                ls = l.split('~')
                fn_name = ls[1]
                var = ls[2]
                val_ty = ls[3]
                mod = ''
                for p in ls[0].split('::'):
                    if 'tests_llm_16_' in p or 'tests_rug_' in p:
                        mod = p
                if mod not in tys:
                    tys[mod] = {}
                if fn_name not in tys[mod]:
                    tys[mod][fn_name] = []
                if (var, val_ty) not in tys[mod][fn_name]:
                    tys[mod][fn_name].append((var, val_ty))
                else:
                    print('err', mod, fn_name, var, val_ty, tys[mod])
            for f in files:
                print(f)
                with open(f, 'r+') as fp:
                    origins = fp.readlines()
                    mutate = copy.deepcopy(origins)
                    i = 0
                    while i < len(mutate):
                        l = mutate[i]
                        if ('let _rug_st_tests_llm_16_' in l or 'let _rug_st_tests_rug_' in l) and '= 0;' in l:
                            # found interest
                            uid+=1
                            name = ''
                            for n in l.split(' '):
                                if '_rug_st_tests_llm_16_' in n or '_rug_st_tests_rug_' in n:
                                    name = n
                            names = name[8:].split('_rrrruuuugggg_')
                            mod = names[0]
                            fn = names[1]
                            if mod not in tys or fn not in tys[mod]:
                                i+=1
                                continue
                            ty = []
                            scalar = []
                            d = []
                            bts = {}
                            for (var, var_ty) in tys[mod][fn]:
                                ud = data[mod][fn][var]
                                bd = get_bytes(var_ty, ud)
                                if bd is not None:
                                    d.append(bd)
                                if var_ty.startswith('&[u8'):
                                    ty.append(var_ty[1:])
                                    scalar.append('mut '+var+'_ext')
                                    bts[var] = var+'_ext'
                                else:
                                    ty.append(var_ty)
                                    scalar.append('mut '+var)
                            if not os.path.exists(fd+'/'+path+'/inputs/{}/{}'.format(mod, fn)):
                                os.makedirs(fd+'/'+path+'/inputs/{}/{}'.format(mod, fn))
                            with open(fd+'/'+path+'/inputs/{}/{}/init{}'.format(mod, fn, uid), 'wb') as bp:
                                for b in d:
                                    bp.write(b)

                            mutate[i] = head.format( ", ".join(scalar), ", ".join(ty))
                            j = i + 1
                            while j<len(mutate):
                                lj = mutate[j]
                                if 'let rug_fuzz_' in lj:
                                    for k,v in bts.items():
                                        if k in lj:
                                            mutate[j] = 'let {} = & {};\n'.format(k, v)
                                            j+=1
                                            continue
                                    mutate[j] = ''
                                if ('let _rug_ed_tests_llm_16_' in lj or 'let _rug_ed_tests_rug_' in lj) and '= 0;' in lj:
                                    mutate[j] = tail
                                    # check if work
                                    fp.truncate(0)
                                    fp.seek(0)
                                    fp.writelines(mutate)
                                    fp.flush()
                                    os.fsync(fp.fileno())
                                    ret = subprocess.run("cargo test --no-run", shell=True, cwd=fd+'/'+path, capture_output=True)
                                    if ret.returncode !=0 :
                                        print('err', mod, fn)
                                        print(ret.stderr.decode('utf-8'))
                                        for p in range(i, j+1):
                                            mutate[p] = origins[p]
                                    break
                                j+=1
                            i = j
                        i += 1
                    fp.truncate(0)
                    fp.seek(0)
                    fp.writelines(mutate)
                    fp.flush()
                    os.fsync(fp.fileno())
            print('done')






if __name__ == '__main__':
    execute_one_fd((sys.argv[1]))
    # args = []
    # for fd in os.listdir('.'):
    #     if not os.path.isdir(fd):
    #         continue
    #     args.append((fd))
    # print(args)
    # with multiprocessing.Pool(24) as p:
    #     p.map(execute_one_fd, args)
