; ModuleID = '/home/onnuri/projects/logai/logai.ll'
source_filename = "logai"
target datalayout = "e-i64:64-i128:128-v16:16-v32:32-n16:32:64"
target triple = "nvptx64-nvidia-cuda"

; Function Attrs: convergent mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: write)
define ptx_kernel void @generate_rng(ptr writeonly captures(none) %v0, i64 %v1, i32 %v2) local_unnamed_addr #0 {
entry:
  %v2.i = tail call i32 @llvm.nvvm.read.ptx.sreg.tid.x() #4
  %v3.i = zext nneg i32 %v2.i to i64
  %v4.i = tail call i32 @llvm.nvvm.read.ptx.sreg.ctaid.x() #4
  %v5.i = zext nneg i32 %v4.i to i64
  %v6.i = tail call i32 @llvm.nvvm.read.ptx.sreg.ntid.x() #4
  %v7.i = zext nneg i32 %v6.i to i64
  %v8.i = mul nuw nsw i64 %v5.i, %v7.i
  %v9.i = add nuw nsw i64 %v8.i, %v3.i
  %v29.not = icmp ult i64 %v9.i, %v1
  br i1 %v29.not, label %bb3, label %bb4

bb3:                                              ; preds = %entry
  %v45.8.i = add i32 %v2, -1879881855
  %v45.4.i = add i32 %v2, 387276957
  %v45.i = add i32 %v2, -1640531527
  %v11.i = and i64 %v9.i, 4294967295
  %v12.i = mul nuw i64 %v11.i, 3528531795
  %v19.i = lshr i64 %v12.i, 32
  %v16.1.i = mul nuw i64 %v19.i, 3449720151
  %v24.1.i = lshr i64 %v16.1.i, 32
  %v29.1.i = trunc nuw i64 %v24.1.i to i32
  %v32.1.i = xor i32 %v45.i, %v29.1.i
  %v11.2.i = zext i32 %v32.1.i to i64
  %v12.2.i = mul nuw i64 %v11.2.i, 3528531795
  %v12.2.i.masked = and i64 %v12.2.i, 4294967295
  %v45.1.i = add i32 %v2, 1013904242
  %v12.i.masked = and i64 %v12.i, 4294967295
  %v11.1.i = zext i32 %v2 to i64
  %v12.1.i = mul nuw i64 %v11.1.i, 3528531795
  %v19.1.i = lshr i64 %v12.1.i, 32
  %0 = xor i64 %v12.i.masked, %v19.1.i
  %v15.2.i = xor i64 %0, 3144134277
  %v16.2.i = mul nuw i64 %v15.2.i, 3449720151
  %v24.2.i = lshr i64 %v16.2.i, 32
  %v29.215.i = xor i64 %v24.2.i, %v16.1.i
  %v29.2.i = trunc i64 %v29.215.i to i32
  %v32.2.i = xor i32 %v45.1.i, %v29.2.i
  %v11.3.i = zext i32 %v32.2.i to i64
  %v12.3.i = mul nuw i64 %v11.3.i, 3528531795
  %v19.3.i = lshr i64 %v12.3.i, 32
  %1 = xor i64 %v12.2.i.masked, %v19.3.i
  %v15.4.i = xor i64 %1, 842468239
  %v16.4.i = mul nuw i64 %v15.4.i, 3449720151
  %v12.3.i.masked = and i64 %v12.3.i, 4294967295
  %v45.2.i = add i32 %v2, -626627285
  %v12.1.i.masked = and i64 %v12.1.i, 4294967295
  %v19.2.i = lshr i64 %v12.2.i, 32
  %2 = xor i64 %v12.1.i.masked, %v19.2.i
  %v15.3.i = xor i64 %2, 1993301258
  %v16.3.i = mul nuw i64 %v15.3.i, 3449720151
  %v24.3.i = lshr i64 %v16.3.i, 32
  %v29.316.i = xor i64 %v24.3.i, %v16.2.i
  %v29.3.i = trunc i64 %v29.316.i to i32
  %v32.3.i = xor i32 %v45.2.i, %v29.3.i
  %v11.4.i = zext i32 %v32.3.i to i64
  %v12.4.i = mul nuw i64 %v11.4.i, 3528531795
  %v19.4.i = lshr i64 %v12.4.i, 32
  %3 = xor i64 %v12.3.i.masked, %v19.4.i
  %v15.5.i = xor i64 %3, 3986602516
  %v16.5.i = mul nuw i64 %v15.5.i, 3449720151
  %v24.5.i = lshr i64 %v16.5.i, 32
  %v29.518.i = xor i64 %v24.5.i, %v16.4.i
  %v29.5.i = trunc i64 %v29.518.i to i32
  %v32.5.i = xor i32 %v45.4.i, %v29.5.i
  %v11.6.i = zext i32 %v32.5.i to i64
  %v12.6.i = mul nuw i64 %v11.6.i, 3528531795
  %v45.5.i = add i32 %v2, -1253254570
  %v12.4.i.masked = and i64 %v12.4.i, 4294967295
  %v45.3.i = add i32 %v2, 2027808484
  %v24.4.i = lshr i64 %v16.4.i, 32
  %v29.417.i = xor i64 %v24.4.i, %v16.3.i
  %v29.4.i = trunc i64 %v29.417.i to i32
  %v32.4.i = xor i32 %v45.3.i, %v29.4.i
  %v11.5.i = zext i32 %v32.4.i to i64
  %v12.5.i = mul nuw i64 %v11.5.i, 3528531795
  %v19.5.i = lshr i64 %v12.5.i, 32
  %4 = xor i64 %v12.4.i.masked, %v19.5.i
  %v15.6.i = xor i64 %4, 2835769497
  %v16.6.i = mul nuw i64 %v15.6.i, 3449720151
  %v24.6.i = lshr i64 %v16.6.i, 32
  %v29.619.i = xor i64 %v24.6.i, %v16.5.i
  %v29.6.i = trunc i64 %v29.619.i to i32
  %v32.6.i = xor i32 %v45.5.i, %v29.6.i
  %v11.7.i = zext i32 %v32.6.i to i64
  %v12.7.i = mul nuw i64 %v11.7.i, 3528531795
  %v19.7.i = lshr i64 %v12.7.i, 32
  %5 = xor i64 %v12.6.i, %v19.7.i
  %v15.8.i = xor i64 %5, 534103459
  %v16.8.i = mul i64 %v15.8.i, 3449720151
  %v12.7.i.masked = and i64 %v12.7.i, 4294967295
  %v45.6.i = add i32 %v2, 1401181199
  %v12.5.i.masked = and i64 %v12.5.i, 4294967295
  %v19.6.i = lshr i64 %v12.6.i, 32
  %6 = xor i64 %v12.5.i.masked, %v19.6.i
  %v15.7.i = xor i64 %6, 1684936478
  %v16.7.i = mul nuw i64 %v15.7.i, 3449720151
  %v24.7.i = lshr i64 %v16.7.i, 32
  %v29.720.i = xor i64 %v24.7.i, %v16.6.i
  %v29.7.i = trunc i64 %v29.720.i to i32
  %v32.7.i = xor i32 %v45.6.i, %v29.7.i
  %v11.8.i = zext i32 %v32.7.i to i64
  %v12.8.i = mul nuw i64 %v11.8.i, 3528531795
  %v19.8.i = lshr i64 %v12.8.i, 32
  %7 = xor i64 %v12.7.i.masked, %v19.8.i
  %v15.9.i = xor i64 %7, 3678237736
  %v16.9.i = mul nuw i64 %v15.9.i, 3449720151
  %v24.9.i = lshr i64 %v16.9.i, 32
  %v29.922.i = xor i64 %v24.9.i, %v16.8.i
  %v29.9.i = trunc i64 %v29.922.i to i32
  %v32.9.i = xor i32 %v45.8.i, %v29.9.i
  %v21 = zext i32 %v32.9.i to i64
  %v22 = mul nuw nsw i64 %v21, 42
  %v25 = lshr i64 %v22, 32
  %v26 = trunc nuw nsw i64 %v25 to i32
  %v27 = add nuw nsw i32 %v26, 1
  %v33 = getelementptr inbounds nuw i32, ptr %v0, i64 %v9.i
  store i32 %v27, ptr %v33, align 4
  br label %bb4

bb4:                                              ; preds = %entry, %bb3
  ret void
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare noundef range(i32 0, 1024) i32 @llvm.nvvm.read.ptx.sreg.tid.x() #1

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare noundef range(i32 0, 2147483647) i32 @llvm.nvvm.read.ptx.sreg.ctaid.x() #1

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare noundef range(i32 1, 1025) i32 @llvm.nvvm.read.ptx.sreg.ntid.x() #1

; Function Attrs: convergent mustprogress nofree norecurse nosync nounwind willreturn memory(none)
define range(i64 0, 2199023254528) i64 @cuda_device____internal__index_1d(ptr readnone captures(none) %v0) local_unnamed_addr #2 {
entry:
  %v2 = tail call i32 @llvm.nvvm.read.ptx.sreg.tid.x() #4
  %v3 = zext nneg i32 %v2 to i64
  %v4 = tail call i32 @llvm.nvvm.read.ptx.sreg.ctaid.x() #4
  %v5 = zext nneg i32 %v4 to i64
  %v6 = tail call i32 @llvm.nvvm.read.ptx.sreg.ntid.x() #4
  %v7 = zext nneg i32 %v6 to i64
  %v8 = mul nuw nsw i64 %v5, %v7
  %v9 = add nuw nsw i64 %v8, %v3
  ret i64 %v9
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none)
define [4 x i32] @logai__kernels__philox4x32([4 x i32] %v0, [2 x i32] %v1) local_unnamed_addr #3 {
entry:
  %v0.fca.0.extract = extractvalue [4 x i32] %v0, 0
  %v0.fca.1.extract = extractvalue [4 x i32] %v0, 1
  %v0.fca.2.extract = extractvalue [4 x i32] %v0, 2
  %v0.fca.3.extract = extractvalue [4 x i32] %v0, 3
  %v1.fca.0.extract = extractvalue [2 x i32] %v1, 0
  %v1.fca.1.extract = extractvalue [2 x i32] %v1, 1
  %v11 = zext i32 %v0.fca.0.extract to i64
  %v12 = mul nuw i64 %v11, 3528531795
  %v15 = zext i32 %v0.fca.2.extract to i64
  %v16 = mul nuw i64 %v15, 3449720151
  %v19 = lshr i64 %v12, 32
  %v20 = trunc nuw i64 %v19 to i32
  %v24 = lshr i64 %v16, 32
  %v25 = trunc nuw i64 %v24 to i32
  %v29 = xor i32 %v0.fca.1.extract, %v25
  %v32 = xor i32 %v29, %v1.fca.0.extract
  %0 = xor i32 %v0.fca.3.extract, %v20
  %v38 = xor i32 %0, %v1.fca.1.extract
  %v45 = add i32 %v1.fca.0.extract, -1640531527
  %v49 = add i32 %v1.fca.1.extract, -1150833019
  %v11.1 = zext i32 %v32 to i64
  %v12.1 = mul nuw i64 %v11.1, 3528531795
  %v15.1 = zext i32 %v38 to i64
  %v16.1 = mul nuw i64 %v15.1, 3449720151
  %v19.1 = lshr i64 %v12.1, 32
  %v24.1 = lshr i64 %v16.1, 32
  %v29.114 = xor i64 %v16, %v24.1
  %v29.1 = trunc i64 %v29.114 to i32
  %v32.1 = xor i32 %v45, %v29.1
  %1 = xor i64 %v12, %v19.1
  %2 = trunc i64 %1 to i32
  %v38.1 = xor i32 %v49, %2
  %v45.1 = add i32 %v1.fca.0.extract, 1013904242
  %v49.1 = add i32 %v1.fca.1.extract, 1993301258
  %v11.2 = zext i32 %v32.1 to i64
  %v12.2 = mul nuw i64 %v11.2, 3528531795
  %v15.2 = zext i32 %v38.1 to i64
  %v16.2 = mul nuw i64 %v15.2, 3449720151
  %v19.2 = lshr i64 %v12.2, 32
  %v24.2 = lshr i64 %v16.2, 32
  %v29.215 = xor i64 %v16.1, %v24.2
  %v29.2 = trunc i64 %v29.215 to i32
  %v32.2 = xor i32 %v45.1, %v29.2
  %3 = xor i64 %v12.1, %v19.2
  %4 = trunc i64 %3 to i32
  %v38.2 = xor i32 %v49.1, %4
  %v45.2 = add i32 %v1.fca.0.extract, -626627285
  %v49.2 = add i32 %v1.fca.1.extract, 842468239
  %v11.3 = zext i32 %v32.2 to i64
  %v12.3 = mul nuw i64 %v11.3, 3528531795
  %v15.3 = zext i32 %v38.2 to i64
  %v16.3 = mul nuw i64 %v15.3, 3449720151
  %v19.3 = lshr i64 %v12.3, 32
  %v24.3 = lshr i64 %v16.3, 32
  %v29.316 = xor i64 %v16.2, %v24.3
  %v29.3 = trunc i64 %v29.316 to i32
  %v32.3 = xor i32 %v45.2, %v29.3
  %5 = xor i64 %v12.2, %v19.3
  %6 = trunc i64 %5 to i32
  %v38.3 = xor i32 %v49.2, %6
  %v45.3 = add i32 %v1.fca.0.extract, 2027808484
  %v49.3 = add i32 %v1.fca.1.extract, -308364780
  %v11.4 = zext i32 %v32.3 to i64
  %v12.4 = mul nuw i64 %v11.4, 3528531795
  %v15.4 = zext i32 %v38.3 to i64
  %v16.4 = mul nuw i64 %v15.4, 3449720151
  %v19.4 = lshr i64 %v12.4, 32
  %v24.4 = lshr i64 %v16.4, 32
  %v29.417 = xor i64 %v16.3, %v24.4
  %v29.4 = trunc i64 %v29.417 to i32
  %v32.4 = xor i32 %v45.3, %v29.4
  %7 = xor i64 %v12.3, %v19.4
  %8 = trunc i64 %7 to i32
  %v38.4 = xor i32 %v49.3, %8
  %v45.4 = add i32 %v1.fca.0.extract, 387276957
  %v49.4 = add i32 %v1.fca.1.extract, -1459197799
  %v11.5 = zext i32 %v32.4 to i64
  %v12.5 = mul nuw i64 %v11.5, 3528531795
  %v15.5 = zext i32 %v38.4 to i64
  %v16.5 = mul nuw i64 %v15.5, 3449720151
  %v19.5 = lshr i64 %v12.5, 32
  %v24.5 = lshr i64 %v16.5, 32
  %v29.518 = xor i64 %v16.4, %v24.5
  %v29.5 = trunc i64 %v29.518 to i32
  %v32.5 = xor i32 %v45.4, %v29.5
  %9 = xor i64 %v12.4, %v19.5
  %10 = trunc i64 %9 to i32
  %v38.5 = xor i32 %v49.4, %10
  %v45.5 = add i32 %v1.fca.0.extract, -1253254570
  %v49.5 = add i32 %v1.fca.1.extract, 1684936478
  %v11.6 = zext i32 %v32.5 to i64
  %v12.6 = mul nuw i64 %v11.6, 3528531795
  %v15.6 = zext i32 %v38.5 to i64
  %v16.6 = mul nuw i64 %v15.6, 3449720151
  %v19.6 = lshr i64 %v12.6, 32
  %v24.6 = lshr i64 %v16.6, 32
  %v29.619 = xor i64 %v16.5, %v24.6
  %v29.6 = trunc i64 %v29.619 to i32
  %v32.6 = xor i32 %v45.5, %v29.6
  %11 = xor i64 %v12.5, %v19.6
  %12 = trunc i64 %11 to i32
  %v38.6 = xor i32 %v49.5, %12
  %v45.6 = add i32 %v1.fca.0.extract, 1401181199
  %v49.6 = add i32 %v1.fca.1.extract, 534103459
  %v11.7 = zext i32 %v32.6 to i64
  %v12.7 = mul nuw i64 %v11.7, 3528531795
  %v15.7 = zext i32 %v38.6 to i64
  %v16.7 = mul nuw i64 %v15.7, 3449720151
  %v19.7 = lshr i64 %v12.7, 32
  %v24.7 = lshr i64 %v16.7, 32
  %v29.720 = xor i64 %v16.6, %v24.7
  %v29.7 = trunc i64 %v29.720 to i32
  %v32.7 = xor i32 %v45.6, %v29.7
  %13 = xor i64 %v12.6, %v19.7
  %14 = trunc i64 %13 to i32
  %v38.7 = xor i32 %v49.6, %14
  %v45.7 = add i32 %v1.fca.0.extract, -239350328
  %v49.7 = add i32 %v1.fca.1.extract, -616729560
  %v11.8 = zext i32 %v32.7 to i64
  %v12.8 = mul nuw i64 %v11.8, 3528531795
  %v15.8 = zext i32 %v38.7 to i64
  %v16.8 = mul nuw i64 %v15.8, 3449720151
  %v19.8 = lshr i64 %v12.8, 32
  %v24.8 = lshr i64 %v16.8, 32
  %v29.821 = xor i64 %v16.7, %v24.8
  %v29.8 = trunc i64 %v29.821 to i32
  %v32.8 = xor i32 %v45.7, %v29.8
  %15 = xor i64 %v12.7, %v19.8
  %16 = trunc i64 %15 to i32
  %v38.8 = xor i32 %v49.7, %16
  %v45.8 = add i32 %v1.fca.0.extract, -1879881855
  %v49.8 = add i32 %v1.fca.1.extract, -1767562579
  %v11.9 = zext i32 %v32.8 to i64
  %v12.9 = mul nuw i64 %v11.9, 3528531795
  %v15.9 = zext i32 %v38.8 to i64
  %v16.9 = mul nuw i64 %v15.9, 3449720151
  %v19.9 = lshr i64 %v12.9, 32
  %v21.9 = trunc i64 %v12.9 to i32
  %v24.9 = lshr i64 %v16.9, 32
  %v26.9 = trunc i64 %v16.9 to i32
  %v29.922 = xor i64 %v16.8, %v24.9
  %v29.9 = trunc i64 %v29.922 to i32
  %v32.9 = xor i32 %v45.8, %v29.9
  %17 = xor i64 %v12.8, %v19.9
  %18 = trunc i64 %17 to i32
  %v38.9 = xor i32 %v49.8, %18
  %v51.fca.0.insert = insertvalue [4 x i32] poison, i32 %v32.9, 0
  %v51.fca.1.insert = insertvalue [4 x i32] %v51.fca.0.insert, i32 %v26.9, 1
  %v51.fca.2.insert = insertvalue [4 x i32] %v51.fca.1.insert, i32 %v38.9, 2
  %v51.fca.3.insert = insertvalue [4 x i32] %v51.fca.2.insert, i32 %v21.9, 3
  ret [4 x i32] %v51.fca.3.insert
}

attributes #0 = { convergent mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: write) }
attributes #1 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #2 = { convergent mustprogress nofree norecurse nosync nounwind willreturn memory(none) }
attributes #3 = { mustprogress nofree norecurse nosync nounwind willreturn memory(none) }
attributes #4 = { convergent }
