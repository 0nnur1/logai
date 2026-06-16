; ModuleID = 'builtin.module'
source_filename = "logai"
target datalayout = "e-i64:64-i128:128-v16:16-v32:32-n16:32:64"
target triple = "nvptx64-nvidia-cuda"

define ptx_kernel void @generate_rng(ptr %v0, i64 %v1, i32 %v2) #0 {
entry:
  %v3 = insertvalue { ptr, i64 } undef, ptr %v0, 0
  %v4 = insertvalue { ptr, i64 } %v3, i64 %v1, 1
  br label %bb0
bb0:
  %v5 = phi { ptr, i64 } [ %v4, %entry ]
  %v6 = phi i32 [ %v2, %entry ]
  %v7 = alloca {  }, align 1
  %v8 = alloca [4 x i32], align 4
  %v9 = bitcast ptr %v7 to ptr
  %v10 = call i64 @cuda_device____internal__index_1d(ptr %v9) #0
  br label %bb1
bb1:
  %v11 = trunc i64 %v10 to i32
  %v12 = insertvalue [4 x i32] undef, i32 %v11, 0
  %v13 = insertvalue [4 x i32] %v12, i32 0, 1
  %v14 = insertvalue [4 x i32] %v13, i32 0, 2
  %v15 = insertvalue [4 x i32] %v14, i32 0, 3
  %v16 = insertvalue [2 x i32] undef, i32 %v6, 0
  %v17 = insertvalue [2 x i32] %v16, i32 0, 1
  %v18 = call [4 x i32] @logai__kernels__philox4x32([4 x i32] %v15, [2 x i32] %v17) #0
  store [4 x i32] %v18, ptr %v8, align 4
  br label %bb2
bb2:
  %v19 = getelementptr inbounds [4 x i32], ptr %v8, i32 0, i64 0
  %v20 = load i32, ptr %v19, align 4
  %v21 = zext i32 %v20 to i64
  %v22 = mul i64 %v21, 42
  %v23 = zext i32 32 to i64
  %v24 = and i64 %v23, 63
  %v25 = lshr i64 %v22, %v24
  %v26 = trunc i64 %v25 to i32
  %v27 = add i32 %v26, 1
  %v28 = extractvalue { ptr, i64 } %v5, 1
  %v29 = icmp ult i64 %v10, %v28
  %v30 = xor i1 %v29, 1
  br i1 %v30, label %bb6, label %bb5
bb3:
  %v31 = extractvalue { i8, ptr } %v37, 1
  store i32 %v27, ptr %v31, align 4
  br label %bb4
bb4:
  ret void
bb5:
  %v32 = extractvalue { ptr, i64 } %v5, 0
  %v33 = getelementptr inbounds i32, ptr %v32, i64 %v10
  %v34 = insertvalue { i8, ptr } undef, i8 1, 0
  %v35 = insertvalue { i8, ptr } %v34, ptr %v33, 1
  br label %bb7
bb6:
  %v36 = insertvalue { i8, ptr } undef, i8 0, 0
  br label %bb7
bb7:
  %v37 = phi { i8, ptr } [ %v35, %bb5 ], [ %v36, %bb6 ]
  %v38 = extractvalue { i8, ptr } %v37, 0
  %v39 = zext i8 %v38 to i64
  %v40 = icmp eq i64 %v39, 1
  br i1 %v40, label %bb3, label %bb8
bb8:
  %v41 = icmp eq i64 %v39, 0
  br i1 %v41, label %bb4, label %bb9
bb9:
  unreachable
}

declare i32 @llvm.nvvm.read.ptx.sreg.tid.x()
declare i32 @llvm.nvvm.read.ptx.sreg.ctaid.x()
declare i32 @llvm.nvvm.read.ptx.sreg.ntid.x()

define i64 @cuda_device____internal__index_1d(ptr %v0) #0 {
entry:
  br label %bb0
bb0:
  %v1 = phi ptr [ %v0, %entry ]
  %v2 = call i32 @llvm.nvvm.read.ptx.sreg.tid.x() #0
  br label %bb1
bb1:
  %v3 = zext i32 %v2 to i64
  %v4 = call i32 @llvm.nvvm.read.ptx.sreg.ctaid.x() #0
  br label %bb2
bb2:
  %v5 = zext i32 %v4 to i64
  %v6 = call i32 @llvm.nvvm.read.ptx.sreg.ntid.x() #0
  br label %bb3
bb3:
  %v7 = zext i32 %v6 to i64
  %v8 = mul i64 %v5, %v7
  %v9 = add i64 %v8, %v3
  ret i64 %v9
}

define [4 x i32] @logai__kernels__philox4x32([4 x i32] %v0, [2 x i32] %v1) #0 {
entry:
  br label %bb0
bb0:
  %v2 = phi [4 x i32] [ %v0, %entry ]
  %v3 = phi [2 x i32] [ %v1, %entry ]
  %v4 = alloca [4 x i32], align 4
  %v5 = alloca [2 x i32], align 4
  store [4 x i32] %v2, ptr %v4, align 4
  store [2 x i32] %v3, ptr %v5, align 4
  br label %bb1
bb1:
  %v6 = phi i32 [ 0, %bb0 ], [ %v60, %bb3 ]
  %v7 = icmp slt i32 %v6, 10
  %v8 = xor i1 %v7, 1
  br i1 %v8, label %bb6, label %bb5
bb2:
  unreachable
bb3:
  %v9 = getelementptr inbounds [4 x i32], ptr %v4, i32 0, i64 0
  %v10 = load i32, ptr %v9, align 4
  %v11 = zext i32 %v10 to i64
  %v12 = mul i64 %v11, 3528531795
  %v13 = getelementptr inbounds [4 x i32], ptr %v4, i32 0, i64 2
  %v14 = load i32, ptr %v13, align 4
  %v15 = zext i32 %v14 to i64
  %v16 = mul i64 %v15, 3449720151
  %v17 = zext i32 32 to i64
  %v18 = and i64 %v17, 63
  %v19 = lshr i64 %v12, %v18
  %v20 = trunc i64 %v19 to i32
  %v21 = trunc i64 %v12 to i32
  %v22 = zext i32 32 to i64
  %v23 = and i64 %v22, 63
  %v24 = lshr i64 %v16, %v23
  %v25 = trunc i64 %v24 to i32
  %v26 = trunc i64 %v16 to i32
  %v27 = getelementptr inbounds [4 x i32], ptr %v4, i32 0, i64 1
  %v28 = load i32, ptr %v27, align 4
  %v29 = xor i32 %v25, %v28
  %v30 = getelementptr inbounds [2 x i32], ptr %v5, i32 0, i64 0
  %v31 = load i32, ptr %v30, align 4
  %v32 = xor i32 %v29, %v31
  %v33 = getelementptr inbounds [4 x i32], ptr %v4, i32 0, i64 3
  %v34 = load i32, ptr %v33, align 4
  %v35 = xor i32 %v20, %v34
  %v36 = getelementptr inbounds [2 x i32], ptr %v5, i32 0, i64 1
  %v37 = load i32, ptr %v36, align 4
  %v38 = xor i32 %v35, %v37
  %v39 = insertvalue [4 x i32] undef, i32 %v32, 0
  %v40 = insertvalue [4 x i32] %v39, i32 %v26, 1
  %v41 = insertvalue [4 x i32] %v40, i32 %v38, 2
  %v42 = insertvalue [4 x i32] %v41, i32 %v21, 3
  store [4 x i32] %v42, ptr %v4, align 4
  %v43 = getelementptr inbounds [2 x i32], ptr %v5, i32 0, i64 0
  %v44 = load i32, ptr %v43, align 4
  %v45 = add i32 %v44, 2654435769
  %v46 = getelementptr inbounds [2 x i32], ptr %v5, i32 0, i64 0
  store i32 %v45, ptr %v46, align 4
  %v47 = getelementptr inbounds [2 x i32], ptr %v5, i32 0, i64 1
  %v48 = load i32, ptr %v47, align 4
  %v49 = add i32 %v48, 3144134277
  %v50 = getelementptr inbounds [2 x i32], ptr %v5, i32 0, i64 1
  store i32 %v49, ptr %v50, align 4
  br label %bb1
bb4:
  %v51 = load [4 x i32], ptr %v4, align 4
  ret [4 x i32] %v51
bb5:
  %v52 = add i32 %v6, 1
  %v53 = insertvalue { i32, i1 } undef, i32 %v52, 0
  %v54 = insertvalue { i32, i1 } %v53, i1 0, 1
  %v55 = extractvalue { i32, i1 } %v54, 0
  %v56 = extractvalue { i32, i1 } %v54, 1
  %v57 = xor i1 %v56, 1
  br i1 %v57, label %bb10, label %bb9
bb6:
  %v58 = insertvalue { i32, i32 } undef, i32 0, 0
  br label %bb7
bb7:
  %v59 = phi { i32, i32 } [ %v58, %bb6 ], [ %v66, %bb10 ]
  %v60 = phi i32 [ %v6, %bb6 ], [ %v55, %bb10 ]
  %v61 = extractvalue { i32, i32 } %v59, 0
  %v62 = zext i32 %v61 to i64
  %v63 = icmp eq i64 %v62, 0
  br i1 %v63, label %bb4, label %bb8
bb8:
  %v64 = icmp eq i64 %v62, 1
  br i1 %v64, label %bb3, label %bb2
bb9:
  br label %bb2
bb10:
  %v65 = insertvalue { i32, i32 } undef, i32 1, 0
  %v66 = insertvalue { i32, i32 } %v65, i32 %v6, 1
  br label %bb7
}


attributes #0 = { convergent }
